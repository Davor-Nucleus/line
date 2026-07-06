use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use eframe::egui;
use wasapi::{
    initialize_mta, AudioCaptureClient, AudioRenderClient, DeviceCollection, DeviceEnumerator,
    Direction, StreamMode, WaveFormat,
};

struct DeviceInfo {
    index: u32,
    name: String,
    is_default: bool,
}

struct LoopbackApp {
    devices: Vec<DeviceInfo>,
    selected_capture: usize,
    selected_output: usize,
    running: bool,
    status: String,
    stop_flag: Option<Arc<AtomicBool>>,
}

impl LoopbackApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // On essaye d'initialiser COM pour ce thread (OK si déjà initialisé)
        let _ = initialize_mta();

        let (devices, selected_capture, selected_output, status) = match enumerate_devices() {
            Ok((devs, sel_cap, sel_out)) => {
                let status = if devs.is_empty() {
                    "Aucun périphérique de lecture trouvé".to_string()
                } else {
                    "Prêt".to_string()
                };
                (devs, sel_cap, sel_out, status)
            }
            Err(e) => {
                let status = format!("Erreur lors de l'énumération des périphériques: {e}");
                (Vec::new(), 0, 0, status)
            }
        };

        Self {
            devices,
            selected_capture,
            selected_output,
            running: false,
            status,
            stop_flag: None,
        }
    }

    fn start_capture_thread(&mut self) {
        if self.devices.is_empty() {
            self.status = "Aucun périphérique à capturer".to_string();
            return;
        }

        let capture_index = self.devices[self.selected_capture].index;
        let output_index = self.devices[self.selected_output].index;
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_thread = Arc::clone(&stop_flag);

        self.stop_flag = Some(stop_flag);
        self.running = true;
        self.status = "Capture en cours…".to_string();

        thread::spawn(move || {
            if let Err(e) = audio_loop(capture_index, output_index, stop_flag_thread) {
                eprintln!("Erreur dans le thread audio: {e:?}");
            }
        });
    }
}

impl eframe::App for LoopbackApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Routeur audio (loopback WASAPI)");
            ui.label("Choisissez le périphérique de lecture à capturer et vers quel périphérique l'envoyer.");

            ui.separator();

            if self.devices.is_empty() {
                ui.label("Aucun périphérique de lecture disponible.");
            } else {
                egui::ComboBox::from_label("Périphérique à capturer")
                    .selected_text(&self.devices[self.selected_capture].name)
                    .show_ui(ui, |cb| {
                        for (i, d) in self.devices.iter().enumerate() {
                            cb.selectable_value(
                                &mut self.selected_capture,
                                i,
                                &d.name,
                            );
                        }
                    });

                ui.add_space(10.0);

                egui::ComboBox::from_label("Périphérique de sortie")
                    .selected_text(&self.devices[self.selected_output].name)
                    .show_ui(ui, |cb| {
                        for (i, d) in self.devices.iter().enumerate() {
                            cb.selectable_value(
                                &mut self.selected_output,
                                i,
                                &d.name,
                            );
                        }
                    });
            }

            ui.add_space(10.0);

            if !self.running {
                if ui
                    .button("Démarrer la capture")
                    .clicked()
                {
                    self.start_capture_thread();
                }
            } else {
                if ui.button("Arrêter").clicked() {
                    if let Some(flag) = &self.stop_flag {
                        flag.store(true, Ordering::SeqCst);
                    }
                    self.running = false;
                    self.status = "Arrêt demandé…".to_string();
                }
            }

            ui.separator();
            ui.label(format!("Statut : {}", self.status));
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn enumerate_devices() -> Result<(Vec<DeviceInfo>, usize, usize)> {
    let enumerator = DeviceEnumerator::new()?;
    let default_output = enumerator.get_default_device(&Direction::Render)?;
    let default_output_id = default_output.get_id()?;
    let devices_col: DeviceCollection = enumerator.get_device_collection(&Direction::Render)?;
    let nbr_devices = devices_col.get_nbr_devices()?;

    let mut devices = Vec::new();
    let mut selected_capture = 0usize;
    let mut selected_output = 0usize;

    for i in 0..nbr_devices {
        let dev = devices_col.get_device_at_index(i)?;
        let name = dev.get_friendlyname()?;
        let id = dev.get_id()?;
        let is_default = id == default_output_id;
        if is_default {
            selected_output = i as usize;
        }
        let label = if is_default {
            format!("{name} (par défaut)")
        } else {
            name
        };
        devices.push(DeviceInfo {
            index: i,
            name: label,
            is_default,
        });
    }

    Ok((devices, selected_capture, selected_output))
}

fn audio_loop(capture_index: u32, output_index: u32, stop_flag: Arc<AtomicBool>) -> Result<()> {
    // Initialisation COM pour ce thread
    initialize_mta()
        .ok()
        .map_err(|e| anyhow!("Échec de initialize_mta dans le thread audio: {:?}", e))?;

    let enumerator = DeviceEnumerator::new()?;
    let devices = enumerator.get_device_collection(&Direction::Render)?;
    let nbr_devices = devices.get_nbr_devices()?;

    if capture_index >= nbr_devices {
        return Err(anyhow!(
            "Index de périphérique de capture invalide ({capture_index}), seulement {nbr_devices} périphériques."
        ));
    }

    if output_index >= nbr_devices {
        return Err(anyhow!(
            "Index de périphérique de sortie invalide ({output_index}), seulement {nbr_devices} périphériques."
        ));
    }

    let capture_device = devices.get_device_at_index(capture_index)?;
    let output_device = devices.get_device_at_index(output_index)?;

    // Format utilisé par le périphérique CAPTURÉ en mode partagé
    let mix_format: WaveFormat = capture_device.get_device_format()?;

    // Créer deux AudioClient :
    //    - un pour CAPTURER en loopback sur le périphérique choisi
    //    - un pour RENDRE (lecture) sur le périphérique de sortie choisi
    let mut capture_client = capture_device.get_iaudioclient()?;
    let mut render_client = output_device.get_iaudioclient()?;

    // mode évènementiel partagé (latence modérée, conversion auto possible)
    let buffer_duration_hns: i64 = 200_000; // 20 ms
    let stream_mode = StreamMode::EventsShared {
        autoconvert: true,
        buffer_duration_hns,
    };

    // Initialiser le client de capture en mode "loopback"
    capture_client.initialize_client(&mix_format, &Direction::Capture, &stream_mode)?;
    let capture_event = capture_client.set_get_eventhandle()?;
    let audio_capture: AudioCaptureClient = capture_client.get_audiocaptureclient()?;

    // Initialiser le client de rendu
    render_client.initialize_client(&mix_format, &Direction::Render, &stream_mode)?;
    let render_event = render_client.set_get_eventhandle()?;
    let audio_render: AudioRenderClient = render_client.get_audiorenderclient()?;

    // Démarrer les flux
    capture_client.start_stream()?;
    render_client.start_stream()?;

    // Tampon circulaire où l’on empile les octets capturés
    let mut buffer: VecDeque<u8> = VecDeque::new();

    // Nb d’octets par frame (tous canaux confondus)
    let bytes_per_frame =
        (mix_format.get_nchannels() as usize) * (mix_format.get_bitspersample() as usize / 8);

    while !stop_flag.load(Ordering::SeqCst) {
        // Attendre qu’il y ait des données à CAPTURER
        capture_event.wait_for_event(200)?; // timeout 200 ms

        // Lire tout ce qui est dispo côté capture
        while let Some(frames_in_packet) = audio_capture.get_next_packet_size()? {
            if frames_in_packet == 0 {
                break;
            }

            let bytes_to_read = frames_in_packet as usize * bytes_per_frame;
            let mut temp = vec![0u8; bytes_to_read];
            let (frames_read, _info) = audio_capture.read_from_device(&mut temp)?;
            let bytes_read = frames_read as usize * bytes_per_frame;
            buffer.extend(&temp[..bytes_read]);
        }

        // Attendre un créneau côté rendu
        render_event.wait_for_event(200)?;

        // Voir combien de frames libres il y a côté sortie
        let free_frames = render_client.get_available_space_in_frames()? as usize;
        if free_frames == 0 {
            thread::sleep(Duration::from_millis(5));
            continue;
        }

        // Nombre de frames qu’on peut réellement écrire en fonction
        // de ce qu’on a dans le tampon
        let max_frames_from_buffer = buffer.len() / bytes_per_frame;
        let frames_to_write = free_frames.min(max_frames_from_buffer);

        if frames_to_write == 0 {
            // Rien à écrire pour le moment
            continue;
        }

        let bytes_to_write = frames_to_write * bytes_per_frame;

        // Construire un slice contigu à partir de la deque
        let mut temp_out = vec![0u8; bytes_to_write];
        for b in temp_out.iter_mut() {
            if let Some(v) = buffer.pop_front() {
                *b = v;
            } else {
                break;
            }
        }

        audio_render.write_to_device(frames_to_write, &temp_out, None)?;
    }

    // Arrêt propre des flux
    let _ = capture_client.stop_stream();
    let _ = render_client.stop_stream();

    Ok(())
}

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Routeur audio (loopback WASAPI)",
        options,
        Box::new(|cc| Ok(Box::new(LoopbackApp::new(cc)))),
    )
}