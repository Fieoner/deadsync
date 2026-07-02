use crate::config::{self};
use crate::game::song::get_song_cache;
use deadlib_platform::dirs;
use deadsync_audio_decode as decode;
use deadsync_chart::{GameplayChartData, SongData};
use deadsync_simfile::cache::{
    SerializableSongData, build_requested_gameplay_charts, build_song_meta,
    load_gameplay_charts_cache_file, load_song_cache_file, song_cache_path, write_song_cache_file,
};
use deadsync_simfile::media::{
    BG_ANIMATIONS_DIR, RANDOM_MOVIES_DIR, SONG_MOVIES_DIR, collect_media_roots,
};
use deadsync_simfile::song::{ParseSongOptions, parse_song_data_file};
use log::{debug, info, warn};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use std::time::Instant;

mod scan;

pub(crate) use scan::collect_song_scan_roots;
pub use scan::{reload_song_dirs_with_progress_counts, scan_and_load_songs_with_progress_counts};

/// Returns true when the pack (song-folder group) that owns this simfile is
/// listed in `NeverCacheList` and so must skip the on-disk cache entirely.
///
/// The group folder is the simfile's pack directory, i.e. the parent of the
/// song directory: `.../Songs/<Group>/<Song>/file.sm`.
fn song_group_is_never_cached(simfile_path: &Path) -> bool {
    simfile_path
        .parent()
        .and_then(Path::parent)
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .is_some_and(config::group_is_never_cached)
}

<<<<<<< HEAD
pub(super) fn compute_song_cache_path(path: &Path) -> Option<PathBuf> {
    let cache_dir = dirs::app_dirs().song_cache_dir();
    match song_cache_path(&cache_dir, path) {
        Ok(path) => Some(path),
        Err(error) => {
=======
impl From<&rssp::stats::ArrowStats> for CachedArrowStats {
    fn from(stats: &rssp::stats::ArrowStats) -> Self {
        Self {
            total_arrows: stats.total_arrows,
            left: stats.left,
            down: stats.down,
            up: stats.up,
            right: stats.right,
            total_steps: stats.total_steps,
            jumps: stats.jumps,
            hands: stats.hands,
            mines: stats.mines,
            holds: stats.holds,
            rolls: stats.rolls,
            lifts: stats.lifts,
            fakes: stats.fakes,
            holding: stats.holding,
        }
    }
}

impl From<CachedArrowStats> for rssp::stats::ArrowStats {
    fn from(stats: CachedArrowStats) -> Self {
        Self {
            total_arrows: stats.total_arrows,
            left: stats.left,
            down: stats.down,
            up: stats.up,
            right: stats.right,
            total_steps: stats.total_steps,
            jumps: stats.jumps,
            hands: stats.hands,
            mines: stats.mines,
            holds: stats.holds,
            rolls: stats.rolls,
            lifts: stats.lifts,
            fakes: stats.fakes,
            holding: stats.holding,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Encode, Decode)]
struct CachedTechCounts {
    pub crossovers: u32,
    pub half_crossovers: u32,
    pub full_crossovers: u32,
    pub footswitches: u32,
    pub up_footswitches: u32,
    pub down_footswitches: u32,
    pub sideswitches: u32,
    pub jacks: u32,
    pub brackets: u32,
    pub doublesteps: u32,
}

impl From<&rssp::TechCounts> for CachedTechCounts {
    fn from(counts: &rssp::TechCounts) -> Self {
        Self {
            crossovers: counts.crossovers,
            half_crossovers: counts.half_crossovers,
            full_crossovers: counts.full_crossovers,
            footswitches: counts.footswitches,
            up_footswitches: counts.up_footswitches,
            down_footswitches: counts.down_footswitches,
            sideswitches: counts.sideswitches,
            jacks: counts.jacks,
            brackets: counts.brackets,
            doublesteps: counts.doublesteps,
        }
    }
}

impl From<CachedTechCounts> for rssp::TechCounts {
    fn from(counts: CachedTechCounts) -> Self {
        Self {
            crossovers: counts.crossovers,
            half_crossovers: counts.half_crossovers,
            full_crossovers: counts.full_crossovers,
            footswitches: counts.footswitches,
            up_footswitches: counts.up_footswitches,
            down_footswitches: counts.down_footswitches,
            sideswitches: counts.sideswitches,
            jacks: counts.jacks,
            brackets: counts.brackets,
            doublesteps: counts.doublesteps,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Encode, Decode)]
struct CachedStaminaCounts {
    pub anchors: u32,
    pub triangles: u32,
    pub boxes: u32,
    pub towers: u32,
    pub doritos: u32,
    pub hip_breakers: u32,
    pub copters: u32,
    pub spirals: u32,
    pub candles: u32,
    pub candle_percent: f64,
    pub staircases: u32,
    pub mono: u32,
    pub mono_percent: f64,
    pub sweeps: u32,
}

impl From<&StaminaCounts> for CachedStaminaCounts {
    fn from(counts: &StaminaCounts) -> Self {
        Self {
            anchors: counts.anchors,
            triangles: counts.triangles,
            boxes: counts.boxes,
            towers: counts.towers,
            doritos: counts.doritos,
            hip_breakers: counts.hip_breakers,
            copters: counts.copters,
            spirals: counts.spirals,
            candles: counts.candles,
            candle_percent: counts.candle_percent,
            staircases: counts.staircases,
            mono: counts.mono,
            mono_percent: counts.mono_percent,
            sweeps: counts.sweeps,
        }
    }
}

impl From<CachedStaminaCounts> for StaminaCounts {
    fn from(counts: CachedStaminaCounts) -> Self {
        Self {
            anchors: counts.anchors,
            triangles: counts.triangles,
            boxes: counts.boxes,
            towers: counts.towers,
            doritos: counts.doritos,
            hip_breakers: counts.hip_breakers,
            copters: counts.copters,
            spirals: counts.spirals,
            candles: counts.candles,
            candle_percent: counts.candle_percent,
            staircases: counts.staircases,
            mono: counts.mono,
            mono_percent: counts.mono_percent,
            sweeps: counts.sweeps,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Encode, Decode)]
enum CachedSpeedUnit {
    Beats,
    Seconds,
}

impl From<SpeedUnit> for CachedSpeedUnit {
    fn from(unit: SpeedUnit) -> Self {
        match unit {
            SpeedUnit::Beats => Self::Beats,
            SpeedUnit::Seconds => Self::Seconds,
        }
    }
}

impl From<CachedSpeedUnit> for SpeedUnit {
    fn from(unit: CachedSpeedUnit) -> Self {
        match unit {
            CachedSpeedUnit::Beats => Self::Beats,
            CachedSpeedUnit::Seconds => Self::Seconds,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Encode, Decode)]
struct CachedSpeedSegment {
    beat: f32,
    ratio: f32,
    delay: f32,
    unit: CachedSpeedUnit,
}

impl From<&SpeedSegment> for CachedSpeedSegment {
    fn from(segment: &SpeedSegment) -> Self {
        Self {
            beat: segment.beat,
            ratio: segment.ratio,
            delay: segment.delay,
            unit: segment.unit.into(),
        }
    }
}

impl From<CachedSpeedSegment> for SpeedSegment {
    fn from(segment: CachedSpeedSegment) -> Self {
        Self {
            beat: segment.beat,
            ratio: segment.ratio,
            delay: segment.delay,
            unit: segment.unit.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
struct CachedTimingSegments {
    beat0_offset_adjust: f32,
    bpms: Vec<(f32, f32)>,
    stops: Vec<(f32, f32)>,
    delays: Vec<(f32, f32)>,
    warps: Vec<(f32, f32)>,
    speeds: Vec<CachedSpeedSegment>,
    scrolls: Vec<(f32, f32)>,
    fakes: Vec<(f32, f32)>,
}

impl From<&TimingSegments> for CachedTimingSegments {
    fn from(segments: &TimingSegments) -> Self {
        Self {
            beat0_offset_adjust: segments.beat0_offset_adjust,
            bpms: segments.bpms.clone(),
            stops: segments
                .stops
                .iter()
                .map(|seg| (seg.beat, seg.duration))
                .collect(),
            delays: segments
                .delays
                .iter()
                .map(|seg| (seg.beat, seg.duration))
                .collect(),
            warps: segments
                .warps
                .iter()
                .map(|seg| (seg.beat, seg.length))
                .collect(),
            speeds: segments
                .speeds
                .iter()
                .map(CachedSpeedSegment::from)
                .collect(),
            scrolls: segments
                .scrolls
                .iter()
                .map(|seg| (seg.beat, seg.ratio))
                .collect(),
            fakes: segments
                .fakes
                .iter()
                .map(|seg| (seg.beat, seg.length))
                .collect(),
        }
    }
}

impl From<CachedTimingSegments> for TimingSegments {
    fn from(segments: CachedTimingSegments) -> Self {
        Self {
            beat0_offset_adjust: segments.beat0_offset_adjust,
            bpms: segments.bpms,
            stops: segments
                .stops
                .into_iter()
                .map(|(beat, duration)| StopSegment { beat, duration })
                .collect(),
            delays: segments
                .delays
                .into_iter()
                .map(|(beat, duration)| DelaySegment { beat, duration })
                .collect(),
            warps: segments
                .warps
                .into_iter()
                .map(|(beat, length)| WarpSegment { beat, length })
                .collect(),
            speeds: segments
                .speeds
                .into_iter()
                .map(SpeedSegment::from)
                .collect(),
            scrolls: segments
                .scrolls
                .into_iter()
                .map(|(beat, ratio)| ScrollSegment { beat, ratio })
                .collect(),
            fakes: segments
                .fakes
                .into_iter()
                .map(|(beat, length)| FakeSegment { beat, length })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Encode, Decode)]
enum CachedNoteType {
    Tap,
    Hold,
    Roll,
    Mine,
    Lift,
    Fake,
}

impl From<NoteType> for CachedNoteType {
    fn from(note_type: NoteType) -> Self {
        match note_type {
            NoteType::Tap => Self::Tap,
            NoteType::Hold => Self::Hold,
            NoteType::Roll => Self::Roll,
            NoteType::Mine => Self::Mine,
            NoteType::Lift => Self::Lift,
            NoteType::Fake => Self::Fake,
        }
    }
}

impl From<CachedNoteType> for NoteType {
    fn from(note_type: CachedNoteType) -> Self {
        match note_type {
            CachedNoteType::Tap => Self::Tap,
            CachedNoteType::Hold => Self::Hold,
            CachedNoteType::Roll => Self::Roll,
            CachedNoteType::Mine => Self::Mine,
            CachedNoteType::Lift => Self::Lift,
            CachedNoteType::Fake => Self::Fake,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
struct CachedParsedNote {
    row_index: u32,
    column: u8,
    note_type: CachedNoteType,
    tail_row_index: Option<u32>,
}

impl From<&ParsedNote> for CachedParsedNote {
    fn from(note: &ParsedNote) -> Self {
        Self {
            row_index: note.row_index as u32,
            column: note.column as u8,
            note_type: note.note_type.into(),
            tail_row_index: note.tail_row_index.map(|v| v as u32),
        }
    }
}

impl From<CachedParsedNote> for ParsedNote {
    fn from(note: CachedParsedNote) -> Self {
        Self {
            row_index: note.row_index as usize,
            column: note.column as usize,
            note_type: note.note_type.into(),
            tail_row_index: note.tail_row_index.map(|v| v as usize),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
struct SerializableChartData {
    chart_type: String,
    difficulty: String,
    description: String,
    chart_name: String,
    meter: u32,
    step_artist: String,
    notes: Vec<u8>,
    parsed_notes: Vec<CachedParsedNote>,
    row_to_beat: Vec<f32>,
    timing_segments: CachedTimingSegments,
    short_hash: String,
    stats: CachedArrowStats,
    tech_counts: CachedTechCounts,
    mines_nonfake: u32,
    stamina_counts: CachedStaminaCounts,
    total_streams: u32,
    max_nps: f64,
    sn_detailed_breakdown: String,
    sn_partial_breakdown: String,
    sn_simple_breakdown: String,
    detailed_breakdown: String,
    partial_breakdown: String,
    simple_breakdown: String,
    chart_attacks: Option<String>,
    chart_bpms: Option<String>,
    chart_stops: Option<String>,
    chart_delays: Option<String>,
    chart_warps: Option<String>,
    chart_speeds: Option<String>,
    chart_scrolls: Option<String>,
    chart_fakes: Option<String>,
    total_measures: usize,
    measure_nps_vec: Vec<f64>,
}

impl From<&ChartData> for SerializableChartData {
    fn from(chart: &ChartData) -> Self {
        Self {
            chart_type: chart.chart_type.clone(),
            difficulty: chart.difficulty.clone(),
            description: chart.description.clone(),
            chart_name: chart.chart_name.clone(),
            meter: chart.meter,
            step_artist: chart.step_artist.clone(),
            notes: chart.notes.clone(),
            parsed_notes: chart
                .parsed_notes
                .iter()
                .map(CachedParsedNote::from)
                .collect(),
            row_to_beat: chart.row_to_beat.clone(),
            timing_segments: (&chart.timing_segments).into(),
            short_hash: chart.short_hash.clone(),
            stats: (&chart.stats).into(),
            tech_counts: (&chart.tech_counts).into(),
            mines_nonfake: chart.mines_nonfake,
            stamina_counts: (&chart.stamina_counts).into(),
            total_streams: chart.total_streams,
            max_nps: chart.max_nps,
            sn_detailed_breakdown: chart.sn_detailed_breakdown.clone(),
            sn_partial_breakdown: chart.sn_partial_breakdown.clone(),
            sn_simple_breakdown: chart.sn_simple_breakdown.clone(),
            detailed_breakdown: chart.detailed_breakdown.clone(),
            partial_breakdown: chart.partial_breakdown.clone(),
            simple_breakdown: chart.simple_breakdown.clone(),
            chart_attacks: chart.chart_attacks.clone(),
            chart_bpms: chart.chart_bpms.clone(),
            chart_stops: chart.chart_stops.clone(),
            chart_delays: chart.chart_delays.clone(),
            chart_warps: chart.chart_warps.clone(),
            chart_speeds: chart.chart_speeds.clone(),
            chart_scrolls: chart.chart_scrolls.clone(),
            chart_fakes: chart.chart_fakes.clone(),
            total_measures: chart.total_measures,
            measure_nps_vec: chart.measure_nps_vec.clone(),
        }
    }
}

impl From<SerializableChartData> for ChartData {
    fn from(chart: SerializableChartData) -> Self {
        Self {
            chart_type: chart.chart_type,
            difficulty: chart.difficulty,
            description: chart.description,
            chart_name: chart.chart_name,
            meter: chart.meter,
            step_artist: chart.step_artist,
            notes: chart.notes,
            parsed_notes: chart
                .parsed_notes
                .into_iter()
                .map(ParsedNote::from)
                .collect(),
            row_to_beat: chart.row_to_beat,
            timing_segments: chart.timing_segments.into(),
            timing: TimingData::default(),
            short_hash: chart.short_hash,
            stats: chart.stats.into(),
            tech_counts: chart.tech_counts.into(),
            mines_nonfake: chart.mines_nonfake,
            stamina_counts: chart.stamina_counts.into(),
            total_streams: chart.total_streams,
            max_nps: chart.max_nps,
            sn_detailed_breakdown: chart.sn_detailed_breakdown,
            sn_partial_breakdown: chart.sn_partial_breakdown,
            sn_simple_breakdown: chart.sn_simple_breakdown,
            detailed_breakdown: chart.detailed_breakdown,
            partial_breakdown: chart.partial_breakdown,
            simple_breakdown: chart.simple_breakdown,
            chart_attacks: chart.chart_attacks,
            chart_bpms: chart.chart_bpms,
            chart_stops: chart.chart_stops,
            chart_delays: chart.chart_delays,
            chart_warps: chart.chart_warps,
            chart_speeds: chart.chart_speeds,
            chart_scrolls: chart.chart_scrolls,
            chart_fakes: chart.chart_fakes,
            total_measures: chart.total_measures,
            measure_nps_vec: chart.measure_nps_vec,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
enum SerializableSongBackgroundChangeTarget {
    File(String),
    NoSongBg,
    Random,
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
struct SerializableSongBackgroundChange {
    start_beat: f32,
    target: SerializableSongBackgroundChangeTarget,
}

impl From<&SongBackgroundChange> for SerializableSongBackgroundChange {
    fn from(change: &SongBackgroundChange) -> Self {
        let target = match &change.target {
            SongBackgroundChangeTarget::File(path) => {
                SerializableSongBackgroundChangeTarget::File(path.to_string_lossy().into_owned())
            }
            SongBackgroundChangeTarget::NoSongBg => {
                SerializableSongBackgroundChangeTarget::NoSongBg
            }
            SongBackgroundChangeTarget::Random => SerializableSongBackgroundChangeTarget::Random,
        };
        Self {
            start_beat: change.start_beat,
            target,
        }
    }
}

impl From<SerializableSongBackgroundChange> for SongBackgroundChange {
    fn from(change: SerializableSongBackgroundChange) -> Self {
        let target = match change.target {
            SerializableSongBackgroundChangeTarget::File(path) => {
                SongBackgroundChangeTarget::File(PathBuf::from(path))
            }
            SerializableSongBackgroundChangeTarget::NoSongBg => {
                SongBackgroundChangeTarget::NoSongBg
            }
            SerializableSongBackgroundChangeTarget::Random => SongBackgroundChangeTarget::Random,
        };
        Self {
            start_beat: change.start_beat,
            target,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encode, Decode)]
struct SerializableSongData {
    simfile_path: String,
    title: String,
    subtitle: String,
    translit_title: String,
    translit_subtitle: String,
    artist: String,
    banner_path: Option<String>,
    background_path: Option<String>,
    background_changes: Vec<SerializableSongBackgroundChange>,
    cdtitle_path: Option<String>,
    music_path: Option<String>,
    display_bpm: String,
    offset: f32,
    sample_start: Option<f32>,
    sample_length: Option<f32>,
    min_bpm: f64,
    max_bpm: f64,
    normalized_bpms: String,
    normalized_stops: String,
    normalized_delays: String,
    normalized_warps: String,
    normalized_speeds: String,
    normalized_scrolls: String,
    normalized_fakes: String,
    music_length_seconds: f32,
    total_length_seconds: i32,
    charts: Vec<SerializableChartData>,
}

impl From<&SongData> for SerializableSongData {
    fn from(song: &SongData) -> Self {
        Self {
            simfile_path: song.simfile_path.to_string_lossy().into_owned(),
            title: song.title.clone(),
            subtitle: song.subtitle.clone(),
            translit_title: song.translit_title.clone(),
            translit_subtitle: song.translit_subtitle.clone(),
            artist: song.artist.clone(),
            banner_path: song
                .banner_path
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            background_path: song
                .background_path
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            background_changes: song
                .background_changes
                .iter()
                .map(SerializableSongBackgroundChange::from)
                .collect(),
            cdtitle_path: song
                .cdtitle_path
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            music_path: song
                .music_path
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            display_bpm: song.display_bpm.clone(),
            offset: song.offset,
            sample_start: song.sample_start,
            sample_length: song.sample_length,
            min_bpm: song.min_bpm,
            max_bpm: song.max_bpm,
            normalized_bpms: song.normalized_bpms.clone(),
            normalized_stops: song.normalized_stops.clone(),
            normalized_delays: song.normalized_delays.clone(),
            normalized_warps: song.normalized_warps.clone(),
            normalized_speeds: song.normalized_speeds.clone(),
            normalized_scrolls: song.normalized_scrolls.clone(),
            normalized_fakes: song.normalized_fakes.clone(),
            music_length_seconds: song.music_length_seconds,
            total_length_seconds: song.total_length_seconds,
            charts: song
                .charts
                .iter()
                .map(SerializableChartData::from)
                .collect(),
        }
    }
}

impl From<SerializableSongData> for SongData {
    fn from(song: SerializableSongData) -> Self {
        Self {
            simfile_path: PathBuf::from(song.simfile_path),
            title: song.title,
            subtitle: song.subtitle,
            translit_title: song.translit_title,
            translit_subtitle: song.translit_subtitle,
            artist: song.artist,
            banner_path: song.banner_path.map(PathBuf::from),
            background_path: song.background_path.map(PathBuf::from),
            background_changes: song
                .background_changes
                .into_iter()
                .map(SongBackgroundChange::from)
                .collect(),
            cdtitle_path: song.cdtitle_path.map(PathBuf::from),
            music_path: song.music_path.map(PathBuf::from),
            display_bpm: song.display_bpm,
            offset: song.offset,
            sample_start: song.sample_start,
            sample_length: song.sample_length,
            min_bpm: song.min_bpm,
            max_bpm: song.max_bpm,
            normalized_bpms: song.normalized_bpms,
            normalized_stops: song.normalized_stops,
            normalized_delays: song.normalized_delays,
            normalized_warps: song.normalized_warps,
            normalized_speeds: song.normalized_speeds,
            normalized_scrolls: song.normalized_scrolls,
            normalized_fakes: song.normalized_fakes,
            music_length_seconds: song.music_length_seconds,
            total_length_seconds: song.total_length_seconds,
            charts: song.charts.into_iter().map(ChartData::from).collect(),
            cached_precise_last_second: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode)]
struct CachedSong {
    cache_version: u8,
    rssp_version: String,
    mono_threshold: usize,
    source_hash: u64,
    data: SerializableSongData,
}

// --- CACHING HELPER FUNCTIONS ---

#[derive(Clone)]
struct SongCacheKeys {
    cache_path: Option<PathBuf>,
}

fn get_content_hash(path: &Path) -> Result<u64, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut hasher = XxHash64::with_seed(0);
    // Using a buffer is much more memory-efficient than reading the whole file at once.
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.write(&buffer[..bytes_read]);
    }
    Ok(hasher.finish())
}

fn get_cache_path(simfile_path: &Path) -> Result<PathBuf, std::io::Error> {
    let canonical_path = simfile_path.canonicalize()?;
    let mut hasher = XxHash64::with_seed(0);
    hasher.write(canonical_path.to_string_lossy().as_bytes());
    let path_hash = hasher.finish();

    let cache_dir = Path::new("cache/songs");
    let hash_hex = format!("{path_hash:016x}");
    let shard2 = &hash_hex[..2];
    Ok(cache_dir.join(shard2).join(format!("{hash_hex}.bin")))
}

fn compute_song_cache_keys(path: &Path) -> SongCacheKeys {
    let cache_path = match get_cache_path(path) {
        Ok(p) => Some(p),
        Err(e) => {
>>>>>>> origin/main
            warn!(
                "Could not generate cache path for {path:?}: {error}. Caching disabled for this file."
            );
            None
        }
    }
}

pub(super) fn load_song_from_cache(
    path: &Path,
    cache_path: &Path,
    verify_freshness: bool,
) -> Option<SongData> {
    let song = load_song_cache_file(path, cache_path, verify_freshness)?;
    debug!("Cache hit for: {:?}", path.file_name().unwrap_or_default());
    Some(song)
}

fn write_song_cache(cache_path: &Path, data: &SerializableSongData, global_offset_seconds: f32) {
    if let Err(error) = write_song_cache_file(cache_path, data, global_offset_seconds) {
        warn!(
            "Could not write song cache for {:?}: {error}",
            Path::new(&data.simfile_path)
                .file_name()
                .unwrap_or_default()
        );
    }
}

<<<<<<< HEAD
fn load_gameplay_charts_from_cache(
    song: &SongData,
    requested_chart_ixs: &[usize],
=======
/// Pre-computes `precise_last_second`, then drops the heavy per-note vectors
/// from every chart so the in-memory song cache stays compact.
fn strip_song_note_data(song: &mut SongData) {
    song.cached_precise_last_second = song.compute_precise_last_second();
    for chart in &mut song.charts {
        chart.strip_notes();
    }
}

/// Helper to load a song from cache OR parse it if needed.
/// Returns (`SongData`, `is_cache_hit`).
fn process_song(
    simfile_path: PathBuf,
    fastload: bool,
    cachesongs: bool,
>>>>>>> origin/main
    global_offset_seconds: f32,
    verify_freshness: bool,
) -> Option<Vec<GameplayChartData>> {
    let cache_path = compute_song_cache_path(&song.simfile_path)?;
    let charts = load_gameplay_charts_cache_file(
        song,
        &cache_path,
        requested_chart_ixs,
        global_offset_seconds,
        verify_freshness,
    )?;
    if verify_freshness {
        debug!(
            "Gameplay cache hit for: {:?}",
            song.simfile_path.file_name().unwrap_or_default()
        );
    } else {
        debug!(
            "Gameplay cache hit (no freshness check) for: {:?}",
            song.simfile_path.file_name().unwrap_or_default()
        );
    }
    Some(charts)
}

/// Re-parse one simfile and replace its in-memory song-cache entry.
///
/// This is used after writing sync edits to disk so immediate replays use the
/// updated timing without a full songs rescan.
pub fn reload_song_in_cache(simfile_path: &Path) -> Result<Arc<SongData>, String> {
    let config = config::get();
    let global_offset_seconds = config.global_offset_seconds;
<<<<<<< HEAD
    let cachesongs = config.cachesongs && !song_group_is_never_cached(simfile_path);
    let cache_path = cachesongs
        .then(|| compute_song_cache_path(simfile_path))
        .flatten();
    let song_data = parse_song_and_maybe_write_cache(
        simfile_path,
=======
    let cachesongs = config.cachesongs;
    let (mut song_data, _) = process_song(
        simfile_path.to_path_buf(),
>>>>>>> origin/main
        false,
        cachesongs,
        cache_path.as_deref(),
        global_offset_seconds,
    )?;
    strip_song_note_data(&mut song_data);
    let updated = Arc::new(song_data);

    let mut song_cache = get_song_cache();
    let mut replaced = false;
    for pack in song_cache.iter_mut() {
        for song in &mut pack.songs {
            if song.simfile_path == simfile_path {
                *song = updated.clone();
                replaced = true;
            }
        }
    }
    if !replaced {
        return Err(format!(
            "Song '{}' not found in song cache",
            simfile_path.display()
        ));
    }
    Ok(updated)
}

<<<<<<< HEAD
fn load_gameplay_song_data(
    simfile_path: &Path,
    allow_cache_write: bool,
=======
/// Scans the provided root directory (e.g., "songs/") for simfiles,
/// parses them, and populates the global cache. This should be run once at startup.
#[allow(dead_code)]
pub fn scan_and_load_songs(root_path_str: &'static str) {
    scan_and_load_songs_impl::<fn(usize, usize, &str, &str)>(root_path_str, None);
}

#[allow(dead_code)]
pub fn scan_and_load_songs_with_progress<F>(root_path_str: &'static str, progress: &mut F)
where
    F: FnMut(&str, &str),
{
    let mut with_counts = |_: usize, _: usize, pack: &str, song: &str| progress(pack, song);
    scan_and_load_songs_impl(root_path_str, Some(&mut with_counts));
}

pub fn scan_and_load_songs_with_progress_counts<F>(root_path_str: &'static str, progress: &mut F)
where
    F: FnMut(usize, usize, &str, &str),
{
    scan_and_load_songs_impl(root_path_str, Some(progress));
}

fn collect_song_scan_roots(root_path_str: &str) -> Vec<PathBuf> {
    fn push_unique_root(path: PathBuf, roots: &mut Vec<PathBuf>, keys: &mut Vec<String>) {
        let mut key = path.to_string_lossy().into_owned();
        if cfg!(windows) {
            key.make_ascii_lowercase();
        }
        if keys.iter().any(|existing| existing == &key) {
            return;
        }
        keys.push(key);
        roots.push(path);
    }

    let mut roots = Vec::with_capacity(4);
    let mut keys: Vec<String> = Vec::with_capacity(4);
    let root_path = PathBuf::from(root_path_str);
    if root_path.is_dir() {
        push_unique_root(root_path, &mut roots, &mut keys);
    } else {
        warn!("Songs directory '{root_path_str}' not found.");
    }

    let additional_folders = crate::config::additional_song_folders();
    for raw in additional_folders.split(',') {
        let path = raw.trim();
        if path.is_empty() {
            continue;
        }
        let extra_root = PathBuf::from(path);
        if extra_root.is_dir() {
            push_unique_root(extra_root, &mut roots, &mut keys);
        } else {
            warn!(
                "AdditionalSongFolders entry '{}' is not a directory; skipping.",
                path
            );
        }
    }
    roots
}

fn ci_key(text: &str) -> String {
    text.trim().to_ascii_lowercase()
}

fn song_scan_key(song: &SongScan) -> String {
    song.dir
        .file_name()
        .and_then(|name| name.to_str())
        .map(ci_key)
        .filter(|key| !key.is_empty())
        .unwrap_or_else(|| song.dir.to_string_lossy().to_ascii_lowercase())
}

fn merge_pack_scan(dst: &mut PackScan, mut src: PackScan) {
    dst.dir = src.dir.clone();
    if src.has_pack_ini {
        dst.display_title = src.display_title.clone();
        dst.sort_title = src.sort_title.clone();
        dst.translit_title = src.translit_title.clone();
        dst.series = src.series.clone();
        dst.year = src.year;
        dst.version = src.version;
        dst.has_pack_ini = true;
        dst.sync_pref = src.sync_pref;
    }
    if src.banner_path.is_some() {
        dst.banner_path = src.banner_path.clone();
    }
    if src.background_path.is_some() {
        dst.background_path = src.background_path.clone();
    }

    let mut song_slots = HashMap::with_capacity(dst.songs.len() + src.songs.len());
    for (idx, song) in dst.songs.iter().enumerate() {
        song_slots.insert(song_scan_key(song), idx);
    }
    for song in src.songs.drain(..) {
        let key = song_scan_key(&song);
        if let Some(slot) = song_slots.get(&key).copied() {
            dst.songs[slot] = song;
        } else {
            let slot = dst.songs.len();
            song_slots.insert(key, slot);
            dst.songs.push(song);
        }
    }
}

fn merge_pack_scans(mut packs: Vec<PackScan>) -> Vec<PackScan> {
    let mut merged = Vec::with_capacity(packs.len());
    let mut pack_slots = HashMap::with_capacity(packs.len());

    for pack in packs.drain(..) {
        let key = ci_key(&pack.group_name);
        if key.is_empty() {
            merged.push(pack);
            continue;
        }
        if let Some(slot) = pack_slots.get(&key).copied() {
            merge_pack_scan(&mut merged[slot], pack);
        } else {
            let slot = merged.len();
            pack_slots.insert(key, slot);
            merged.push(pack);
        }
    }

    merged
}

#[inline(always)]
fn report_load_progress<F>(
    progress: &mut Option<&mut F>,
    done: usize,
    total: usize,
    group: &str,
    item: &str,
) where
    F: FnMut(usize, usize, &str, &str),
{
    if let Some(cb) = progress.as_mut() {
        cb(done, total, group, item);
    }
}

#[inline(always)]
fn song_pack_progress_name(pack: &SongPack) -> &str {
    pack.directory
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or(pack.group_name.as_str())
}

#[inline(always)]
fn song_progress_name(path: &Path) -> &str {
    path.parent()
        .and_then(|dir| dir.file_name())
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| {
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
        })
}

#[inline(always)]
fn course_progress_names<'a>(path: &'a Path, root: &'a str) -> (&'a str, &'a str) {
    let group = path
        .parent()
        .and_then(|dir| dir.file_name())
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or(root);
    let course = path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or_default();
    (group, course)
}

type SongParseMsg = (usize, PathBuf, Result<(Arc<SongData>, bool), String>);

fn reap_song_parse<F>(
    rx: Option<&std::sync::mpsc::Receiver<SongParseMsg>>,
    in_flight: &mut usize,
    loaded_packs: &mut Vec<SongPack>,
    songs_failed: &mut usize,
    songs_cache_hits: &mut usize,
    songs_parsed: &mut usize,
    songs_done: &mut usize,
    total_songs: usize,
    progress: &mut Option<&mut F>,
) where
    F: FnMut(usize, usize, &str, &str),
{
    let Some(rx) = rx else {
        return;
    };
    match rx.recv() {
        Ok((pack_idx, simfile_path, result)) => {
            *in_flight = in_flight.saturating_sub(1);
            match result {
                Ok((song_data, is_hit)) => {
                    if is_hit {
                        *songs_cache_hits += 1;
                    } else {
                        *songs_parsed += 1;
                    }
                    if let Some(pack) = loaded_packs.get_mut(pack_idx) {
                        pack.songs.push(song_data);
                    }
                }
                Err(e) => {
                    *songs_failed += 1;
                    warn!("Failed to load '{simfile_path:?}': {e}")
                }
            }
            *songs_done = songs_done.saturating_add(1);
            let pack_display = loaded_packs
                .get(pack_idx)
                .map_or("", song_pack_progress_name);
            report_load_progress(
                progress,
                *songs_done,
                total_songs,
                pack_display,
                song_progress_name(&simfile_path),
            );
        }
        Err(_) => {
            *in_flight = 0;
        }
    }
}

fn scan_and_load_songs_impl<F>(root_path_str: &'static str, mut progress: Option<&mut F>)
where
    F: FnMut(usize, usize, &str, &str),
{
    info!("Starting simfile scan (base songs root '{root_path_str}')...");

    let started = Instant::now();
    let config = crate::config::get();
    let fastload = config.fastload;
    let cachesongs = config.cachesongs;
    let global_offset_seconds = config.global_offset_seconds;

    let avail_threads = std::thread::available_parallelism()
        .map(std::num::NonZero::get)
        .unwrap_or(1);
    let mut parse_threads = match config.song_parsing_threads {
        0 => avail_threads,
        1 => 1,
        n => (n as usize).min(avail_threads).max(1),
    };
    if parse_threads < 1 {
        parse_threads = 1;
    }
    let parallel_parsing = parse_threads > 1;

    // Ensure the cache directory exists before we start scanning.
    let cache_dir = Path::new("cache/songs");
    if let Err(e) = fs::create_dir_all(cache_dir) {
        warn!(
            "Could not create cache directory '{}': {}. Caching will be disabled.",
            cache_dir.to_string_lossy(),
            e
        );
    }

    let song_roots = collect_song_scan_roots(root_path_str);
    if song_roots.is_empty() {
        warn!("No valid song roots found. No songs will be loaded.");
        set_song_cache(Vec::new());
        return;
    }

    let mut loaded_packs = Vec::new();
    let mut songs_cache_hits = 0usize;
    let mut songs_parsed = 0usize;
    let mut songs_failed = 0usize;

    let mut packs = Vec::new();
    for songs_root in &song_roots {
        match rssp::pack::scan_songs_dir(songs_root, rssp::pack::ScanOpt::default()) {
            Ok(mut found) => packs.append(&mut found),
            Err(e) => warn!("Could not scan songs dir '{}': {e:?}", songs_root.display()),
        }
    }
    packs = merge_pack_scans(packs);
    let total_songs = packs.iter().map(|pack| pack.songs.len()).sum::<usize>();
    let mut songs_done = 0usize;
    report_load_progress(&mut progress, 0, total_songs, "", "");

    let mut runtime: Option<tokio::runtime::Runtime> = None;
    let mut tx_opt: Option<std::sync::mpsc::Sender<SongParseMsg>> = None;
    let mut rx_opt: Option<std::sync::mpsc::Receiver<SongParseMsg>> = None;
    let mut in_flight = 0usize;

    for pack in packs {
        let pack_display = pack
            .dir
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(pack.group_name.as_str())
            .to_owned();

        let current_pack = SongPack {
            group_name: pack.group_name,
            name: pack.display_title,
            sort_title: pack.sort_title,
            translit_title: pack.translit_title,
            series: pack.series,
            year: pack.year,
            sync_pref: pack.sync_pref,
            directory: pack.dir,
            banner_path: pack.banner_path,
            songs: Vec::new(),
        };
        debug!("Scanning pack: {}", current_pack.name);
        let pack_idx = loaded_packs.len();
        loaded_packs.push(current_pack);

        for song in pack.songs {
            let simfile_path = song.simfile;
            let song_display = song_progress_name(&simfile_path);

            if parallel_parsing {
                let rt = runtime.get_or_insert_with(|| {
                    tokio::runtime::Builder::new_current_thread()
                        .max_blocking_threads(parse_threads)
                        .build()
                        .unwrap()
                });
                if tx_opt.is_none() || rx_opt.is_none() {
                    let (tx, rx) = std::sync::mpsc::channel::<SongParseMsg>();
                    tx_opt = Some(tx);
                    rx_opt = Some(rx);
                }

                while in_flight >= parse_threads {
                    reap_song_parse(
                        rx_opt.as_ref(),
                        &mut in_flight,
                        &mut loaded_packs,
                        &mut songs_failed,
                        &mut songs_cache_hits,
                        &mut songs_parsed,
                        &mut songs_done,
                        total_songs,
                        &mut progress,
                    );
                }

                let Some(tx) = tx_opt.as_ref() else {
                    // Fallback to sync if channel creation failed (unlikely)
                    match process_song(
                        simfile_path.clone(),
                        fastload,
                        cachesongs,
                        global_offset_seconds,
                    ) {
                        Ok((mut song_data, is_hit)) => {
                            if is_hit {
                                songs_cache_hits += 1;
                            } else {
                                songs_parsed += 1;
                            }
                            strip_song_note_data(&mut song_data);
                            loaded_packs[pack_idx].songs.push(Arc::new(song_data));
                        }
                        Err(e) => {
                            songs_failed += 1;
                            warn!("Failed to load '{simfile_path:?}': {e}")
                        }
                    }
                    songs_done = songs_done.saturating_add(1);
                    report_load_progress(
                        &mut progress,
                        songs_done,
                        total_songs,
                        pack_display.as_str(),
                        song_display,
                    );
                    continue;
                };

                let tx = tx.clone();
                let simfile_path_owned = simfile_path.clone();
                rt.handle().spawn_blocking(move || {
                    let out = catch_unwind(AssertUnwindSafe(|| {
                        process_song(
                            simfile_path_owned.clone(),
                            fastload,
                            cachesongs,
                            global_offset_seconds,
                        )
                        .map(|(mut d, h)| {
                            strip_song_note_data(&mut d);
                            (Arc::new(d), h)
                        })
                    }))
                    .unwrap_or_else(|_| Err("Song parse panicked".to_string()));
                    let _ = tx.send((pack_idx, simfile_path_owned, out));
                });
                in_flight += 1;
            } else {
                match process_song(
                    simfile_path.clone(),
                    fastload,
                    cachesongs,
                    global_offset_seconds,
                ) {
                    Ok((mut song_data, is_hit)) => {
                        if is_hit {
                            songs_cache_hits += 1;
                        } else {
                            songs_parsed += 1;
                        }
                        strip_song_note_data(&mut song_data);
                        loaded_packs[pack_idx].songs.push(Arc::new(song_data));
                    }
                    Err(e) => {
                        songs_failed += 1;
                        warn!("Failed to load '{simfile_path:?}': {e}")
                    }
                }
                songs_done = songs_done.saturating_add(1);
                report_load_progress(
                    &mut progress,
                    songs_done,
                    total_songs,
                    pack_display.as_str(),
                    song_display,
                );
            }
        }
    }

    while in_flight > 0 {
        reap_song_parse(
            rx_opt.as_ref(),
            &mut in_flight,
            &mut loaded_packs,
            &mut songs_failed,
            &mut songs_cache_hits,
            &mut songs_parsed,
            &mut songs_done,
            total_songs,
            &mut progress,
        );
    }

    if runtime.is_some() {
        debug!(
            "Song parsing: used {} threads for cache/parsing (SongParsingThreads={}).",
            parse_threads, config.song_parsing_threads
        );
    }

    loaded_packs.retain(|p| !p.songs.is_empty());
    for pack in &mut loaded_packs {
        pack.songs
            .sort_by_cached_key(|s| ItgmaniaSongTitleKey::new(s.as_ref()));
    }

    loaded_packs.sort_by_cached_key(|p| {
        (
            p.sort_title.to_ascii_lowercase(),
            p.group_name.to_ascii_lowercase(),
        )
    });

    let songs_loaded = loaded_packs.iter().map(|p| p.songs.len()).sum::<usize>();
    info!(
        "Finished scan. Found {} packs / {} songs (parsed {}, cache hits {}, failed {}) in {}.",
        loaded_packs.len(),
        songs_loaded,
        songs_parsed,
        songs_cache_hits,
        songs_failed,
        fmt_scan_time(started.elapsed())
    );
    set_song_cache(loaded_packs);
}

fn is_dir_ci(dir: &Path, name: &str) -> Option<PathBuf> {
    let want = name.trim();
    if want.is_empty() {
        return None;
    }
    let want_ci = want.to_ascii_lowercase();
    let Ok(entries) = fs::read_dir(dir) else {
        return None;
    };
    let mut ci_match = None;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let got = entry.file_name();
        let got = got.to_string_lossy();
        if got == want {
            return Some(path);
        }
        if ci_match.is_none() && got.to_ascii_lowercase() == want_ci {
            ci_match = Some(path);
        }
    }
    ci_match
}

fn collect_course_paths(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("crs"))
            {
                out.push(path);
            }
        }
    }
    out.sort_by_cached_key(|p| p.to_string_lossy().to_ascii_lowercase());
    out
}

fn resolve_song_dir(
    song_roots: &[PathBuf],
    group_dirs: &mut HashMap<String, PathBuf>,
    group: Option<&str>,
    song: &str,
) -> Option<PathBuf> {
    fn resolve_group_dir(
        song_roots: &[PathBuf],
        group_dirs: &mut HashMap<String, PathBuf>,
        group: &str,
    ) -> Option<PathBuf> {
        let key = group.trim().to_ascii_lowercase();
        if key.is_empty() {
            return None;
        }
        if !group_dirs.contains_key(&key) {
            let mut path = None;
            for songs_root in song_roots.iter().rev() {
                path = is_dir_ci(songs_root, group);
                if path.is_some() {
                    break;
                }
            }
            let path = path?;
            group_dirs.insert(key.clone(), path);
        }
        group_dirs.get(&key).cloned()
    }

    let song = song.trim();
    if song.is_empty() {
        return None;
    }

    if let Some(group) = group.map(str::trim).filter(|g| !g.is_empty()) {
        let group_dir = resolve_group_dir(song_roots, group_dirs, group)?;
        return is_dir_ci(&group_dir, song);
    }

    for songs_root in song_roots.iter().rev() {
        let Ok(entries) = fs::read_dir(songs_root) else {
            continue;
        };
        for entry in entries.flatten() {
            let group_dir = entry.path();
            if !group_dir.is_dir() {
                continue;
            }
            if let Some(found) = is_dir_ci(&group_dir, song) {
                return Some(found);
            }
        }
    }
    None
}

fn resolve_course_group_dir(
    song_roots: &[PathBuf],
    group_dirs: &mut HashMap<String, PathBuf>,
    group: &str,
) -> Option<PathBuf> {
    let key = group.trim().to_ascii_lowercase();
    if key.is_empty() {
        return None;
    }
    if let Some(path) = group_dirs.get(&key) {
        return Some(path.clone());
    }
    let mut path = None;
    for songs_root in song_roots.iter().rev() {
        path = is_dir_ci(songs_root, group);
        if path.is_some() {
            break;
        }
    }
    let path = path?;
    group_dirs.insert(key, path.clone());
    Some(path)
}

fn autogen_nonstop_group_courses() -> Vec<(PathBuf, rssp::course::CourseFile)> {
    let song_cache = get_song_cache();
    let mut out = Vec::with_capacity(song_cache.len());

    for pack in song_cache.iter() {
        if pack.songs.is_empty() {
            continue;
        }

        let group_name = pack.group_name.trim();
        if group_name.is_empty() {
            continue;
        }
        let display_name = if pack.name.trim().is_empty() {
            group_name
        } else {
            pack.name.trim()
        };

        let mut entries = Vec::with_capacity(4);
        for _ in 0..4 {
            entries.push(rssp::course::CourseEntry {
                song: rssp::course::CourseSong::RandomWithinGroup {
                    group: group_name.to_string(),
                },
                steps: rssp::course::StepsSpec::Difficulty(rssp::course::Difficulty::Medium),
                modifiers: String::new(),
                secret: true,
                no_difficult: false,
                gain_lives: -1,
            });
        }

        let mut path = PathBuf::from("courses");
        path.push(group_name);
        path.push("__deadsync_autogen_nonstop_random.crs");

        out.push((
            path,
            rssp::course::CourseFile {
                name: format!("{display_name} Random"),
                name_translit: String::new(),
                scripter: "Autogen".to_string(),
                description: String::new(),
                banner: pack
                    .banner_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                background: String::new(),
                repeat: false,
                lives: -1,
                meters: [None; 6],
                entries,
            },
        ));
    }

    out
}

#[allow(dead_code)]
pub fn scan_and_load_courses(courses_root_str: &'static str, songs_root_str: &'static str) {
    scan_and_load_courses_impl::<fn(usize, usize, &str, &str)>(
        courses_root_str,
        songs_root_str,
        None,
    );
}

#[allow(dead_code)]
pub fn scan_and_load_courses_with_progress<F>(
    courses_root_str: &'static str,
    songs_root_str: &'static str,
    progress: &mut F,
) where
    F: FnMut(&str, &str),
{
    let mut with_counts = |_: usize, _: usize, group: &str, course: &str| progress(group, course);
    scan_and_load_courses_impl(courses_root_str, songs_root_str, Some(&mut with_counts));
}

pub fn scan_and_load_courses_with_progress_counts<F>(
    courses_root_str: &'static str,
    songs_root_str: &'static str,
    progress: &mut F,
) where
    F: FnMut(usize, usize, &str, &str),
{
    scan_and_load_courses_impl(courses_root_str, songs_root_str, Some(progress));
}

fn scan_and_load_courses_impl<F>(
    courses_root_str: &'static str,
    songs_root_str: &'static str,
    mut progress: Option<&mut F>,
) where
    F: FnMut(usize, usize, &str, &str),
{
    info!("Starting course scan in '{courses_root_str}'...");
    let started = Instant::now();

    let courses_root = Path::new(courses_root_str);
    if !courses_root.is_dir() {
        warn!("Courses directory '{courses_root_str}' not found. No courses will be loaded.");
        set_course_cache(Vec::new());
        return;
    }

    let song_roots = collect_song_scan_roots(songs_root_str);
    if song_roots.is_empty() {
        warn!("No valid song roots found. No courses will be loaded.");
        set_course_cache(Vec::new());
        return;
    }

    let mut loaded_courses = Vec::new();
    let mut courses_failed = 0usize;
    let mut group_dirs: HashMap<String, PathBuf> = HashMap::new();
    let total_song_count = {
        let song_cache = get_song_cache();
        song_cache
            .iter()
            .map(|pack| pack.songs.len())
            .sum::<usize>()
    };
    let course_paths = collect_course_paths(courses_root);
    let total_courses = course_paths.len();
    let mut courses_done = 0usize;
    report_load_progress(&mut progress, 0, total_courses, "", "");

    for course_path in course_paths {
        let (group_display, course_display) = course_progress_names(&course_path, courses_root_str);
        let group_display = group_display.to_owned();
        let course_display = course_display.to_owned();
        let mut report_done = || {
            courses_done = courses_done.saturating_add(1);
            report_load_progress(
                &mut progress,
                courses_done,
                total_courses,
                &group_display,
                &course_display,
            );
        };
        let data = match fs::read(&course_path) {
            Ok(d) => d,
            Err(e) => {
                courses_failed += 1;
                warn!("Failed to read course '{}': {}", course_path.display(), e);
                report_done();
                continue;
            }
        };

        let course = match rssp::course::parse_crs(&data) {
            Ok(c) => c,
            Err(e) => {
                courses_failed += 1;
                warn!("Failed to parse course '{}': {}", course_path.display(), e);
                report_done();
                continue;
            }
        };

        let mut ok = true;
        for (idx, entry) in course.entries.iter().enumerate() {
            match &entry.song {
                rssp::course::CourseSong::Fixed { group, song } => {
                    let Some(song_dir) =
                        resolve_song_dir(&song_roots, &mut group_dirs, group.as_deref(), song)
                    else {
                        warn!(
                            "Course '{}' entry {} references missing song '{}{}'.",
                            course.name,
                            idx + 1,
                            group
                                .as_deref()
                                .map(|g| format!("{g}/"))
                                .unwrap_or_default(),
                            song
                        );
                        ok = false;
                        break;
                    };

                    match rssp::pack::scan_song_dir(&song_dir, rssp::pack::ScanOpt::default()) {
                        Ok(Some(_)) => {}
                        Ok(None) => {
                            warn!(
                                "Course '{}' entry {} song dir has no simfile: {}",
                                course.name,
                                idx + 1,
                                song_dir.display()
                            );
                            ok = false;
                            break;
                        }
                        Err(e) => {
                            warn!(
                                "Course '{}' entry {} failed scanning song dir {}: {e:?}",
                                course.name,
                                idx + 1,
                                song_dir.display()
                            );
                            ok = false;
                            break;
                        }
                    }
                }
                rssp::course::CourseSong::SortPick { sort, index } => {
                    let supports_sort = matches!(
                        sort,
                        rssp::course::SongSort::MostPlays | rssp::course::SongSort::FewestPlays
                    );
                    if !supports_sort {
                        warn!(
                            "Course '{}' has unsupported sort selector in entry {} ({sort:?}).",
                            course.name,
                            idx + 1,
                        );
                        ok = false;
                        break;
                    }

                    let choose_index = (*index).max(0) as usize;
                    if choose_index >= total_song_count {
                        let label = match sort {
                            rssp::course::SongSort::MostPlays => "BEST",
                            rssp::course::SongSort::FewestPlays => "WORST",
                            rssp::course::SongSort::TopGrades => "GRADEBEST",
                            rssp::course::SongSort::LowestGrades => "GRADEWORST",
                        };
                        warn!(
                            "Course '{}' entry {} references out-of-range sort pick '{}{}' with only {} songs installed.",
                            course.name,
                            idx + 1,
                            label,
                            choose_index.saturating_add(1),
                            total_song_count
                        );
                        ok = false;
                        break;
                    }
                }
                rssp::course::CourseSong::RandomAny => {}
                rssp::course::CourseSong::RandomWithinGroup { group } => {
                    if resolve_course_group_dir(&song_roots, &mut group_dirs, group).is_none() {
                        warn!(
                            "Course '{}' entry {} references missing group '{}/*'.",
                            course.name,
                            idx + 1,
                            group
                        );
                        ok = false;
                        break;
                    }
                }
                _ => {
                    warn!(
                        "Course '{}' has unsupported song selector in entry {}.",
                        course.name,
                        idx + 1
                    );
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            loaded_courses.push((course_path, course));
        } else {
            courses_failed += 1;
        }
        report_done();
    }

    let autogen_courses = autogen_nonstop_group_courses();
    let autogen_count = autogen_courses.len();
    loaded_courses.extend(autogen_courses);

    info!(
        "Finished course scan. Loaded {} courses ({} autogen, failed {}) in {}.",
        loaded_courses.len(),
        autogen_count,
        courses_failed,
        fmt_scan_time(started.elapsed())
    );
    set_course_cache(loaded_courses);
}

fn load_song_from_cache(
    path: &Path,
    cache_path: &Path,
>>>>>>> origin/main
    global_offset_seconds: f32,
) -> Result<SerializableSongData, String> {
    let started = Instant::now();
    let cache_path = allow_cache_write
        .then(|| compute_song_cache_path(simfile_path))
        .flatten();
    let parse_started = Instant::now();
    let song_data = parse_song_cache_data(simfile_path, global_offset_seconds)?;
    let parse_ms = parse_started.elapsed().as_secs_f64() * 1000.0;
    let write_started = Instant::now();
    if allow_cache_write && let Some(cp) = cache_path.as_deref() {
        write_song_cache(cp, &song_data, global_offset_seconds);
    }
    let write_ms = write_started.elapsed().as_secs_f64() * 1000.0;
    let total_ms = started.elapsed().as_secs_f64() * 1000.0;
    if total_ms >= 25.0 {
        info!(
            "Gameplay song data load: source=parse file={:?} parse_ms={parse_ms:.3} write_ms={write_ms:.3} elapsed_ms={total_ms:.3}",
            simfile_path.file_name().unwrap_or_default()
        );
    } else {
        debug!(
            "Gameplay song data load: source=parse file={:?} parse_ms={parse_ms:.3} write_ms={write_ms:.3} elapsed_ms={total_ms:.3}",
            simfile_path.file_name().unwrap_or_default()
        );
    }
    Ok(song_data)
}

pub fn load_gameplay_charts(
    song: &SongData,
    requested_chart_ixs: &[usize],
    global_offset_seconds: f32,
) -> Result<Vec<GameplayChartData>, String> {
    let started = Instant::now();
    let config = config::get();
    let never_cache = song_group_is_never_cached(&song.simfile_path);
    let allow_cache_read = (config.fastload || config.cachesongs) && !never_cache;
    let allow_cache_write = config.cachesongs && !never_cache;
    let verify_cache_freshness = !config.fastload;
    let load_started = Instant::now();
    if allow_cache_read
        && let Some(charts) = load_gameplay_charts_from_cache(
            song,
            requested_chart_ixs,
            global_offset_seconds,
            verify_cache_freshness,
        )
    {
        let load_ms = load_started.elapsed().as_secs_f64() * 1000.0;
        let total_ms = started.elapsed().as_secs_f64() * 1000.0;
        if total_ms >= 25.0 {
            info!(
                "Gameplay chart payload load: song='{}' requested={} load_ms={load_ms:.3} materialize_ms=0.000 elapsed_ms={total_ms:.3}",
                song.title,
                requested_chart_ixs.len()
            );
        } else {
            debug!(
                "Gameplay chart payload load: song='{}' requested={} load_ms={load_ms:.3} materialize_ms=0.000 elapsed_ms={total_ms:.3}",
                song.title,
                requested_chart_ixs.len()
            );
        }
        return Ok(charts);
    }

    let song_data =
        load_gameplay_song_data(&song.simfile_path, allow_cache_write, global_offset_seconds)?;
    let load_ms = load_started.elapsed().as_secs_f64() * 1000.0;
    let build_started = Instant::now();
    let charts =
        build_requested_gameplay_charts(&song_data, requested_chart_ixs, global_offset_seconds)?;
    let build_ms = build_started.elapsed().as_secs_f64() * 1000.0;
    let total_ms = started.elapsed().as_secs_f64() * 1000.0;
    if total_ms >= 25.0 {
        info!(
            "Gameplay chart payload load: song='{}' requested={} load_ms={load_ms:.3} materialize_ms={build_ms:.3} elapsed_ms={total_ms:.3}",
            song.title,
            requested_chart_ixs.len()
        );
    } else {
        debug!(
            "Gameplay chart payload load: song='{}' requested={} load_ms={load_ms:.3} materialize_ms={build_ms:.3} elapsed_ms={total_ms:.3}",
            song.title,
            requested_chart_ixs.len()
        );
    }
    Ok(charts)
}

pub fn load_sync_analysis_chart(
    song: &SongData,
    chart_ix: usize,
) -> Result<GameplayChartData, String> {
    let config = config::get();
    let allow_cache_read =
        (config.fastload || config.cachesongs) && !song_group_is_never_cached(&song.simfile_path);
    let verify_cache_freshness = !config.fastload;
    if allow_cache_read
        && let Some(mut charts) =
            load_gameplay_charts_from_cache(song, &[chart_ix], 0.0, verify_cache_freshness)
        && let Some(chart) = charts.pop()
    {
        return Ok(chart);
    }

    let song_data = load_gameplay_song_data(&song.simfile_path, false, 0.0)?;
    let mut charts = build_requested_gameplay_charts(&song_data, &[chart_ix], 0.0)?;
    charts
        .pop()
        .ok_or_else(|| format!("Chart index {chart_ix} out of range"))
}

pub(super) fn parse_song_and_maybe_write_cache(
    path: &Path,
    fastload: bool,
    cachesongs: bool,
    cache_path: Option<&Path>,
    global_offset_seconds: f32,
) -> Result<SongData, String> {
    if fastload {
        debug!("Cache miss for: {:?}", path.file_name().unwrap_or_default());
    } else {
        debug!(
            "Parsing (fastload disabled): {:?}",
            path.file_name().unwrap_or_default()
        );
    }
    let song_data = parse_song_cache_data(path, global_offset_seconds)?;
    if cachesongs && let Some(cp) = cache_path {
        write_song_cache(cp, &song_data, global_offset_seconds);
    }
    Ok(build_song_meta(song_data, global_offset_seconds))
}

#[cfg(test)]
pub(crate) fn parse_song_for_test(
    path: &Path,
    global_offset_seconds: f32,
) -> Result<SongData, String> {
    deadsync_simfile::song::parse_song_meta_file(
        path,
        &parse_song_options(),
        global_offset_seconds,
        compute_music_length_seconds,
    )
}

fn bgchange_asset_roots(dirname: &str) -> Vec<PathBuf> {
    let dirs = dirs::app_dirs();
    let cwd = std::env::current_dir().ok();
    collect_media_roots(dirname, &dirs.data_dir, &dirs.exe_dir, cwd.as_deref())
}

fn parse_song_options() -> ParseSongOptions {
    ParseSongOptions::new(
        bgchange_asset_roots(SONG_MOVIES_DIR),
        bgchange_asset_roots(RANDOM_MOVIES_DIR),
        bgchange_asset_roots(BG_ANIMATIONS_DIR),
    )
}

<<<<<<< HEAD
/// Parse and normalize a simfile on a cache miss.
fn parse_song_cache_data(
    path: &Path,
    global_offset_seconds: f32,
) -> Result<SerializableSongData, String> {
    parse_song_data_file(
        path,
        &parse_song_options(),
        global_offset_seconds,
        compute_music_length_seconds,
    )
=======
    let (banner_path, background_path_opt) = rssp::assets::resolve_song_assets(
        simfile_dir,
        &summary.banner_path,
        &summary.background_path,
    );
    let background_changes =
        rssp::assets::resolve_background_changes_like_itg(simfile_dir, &simfile_data)
            .into_iter()
            .map(convert_background_change)
            .collect();
    let cdtitle_path = resolve_song_asset_path_like_itg(simfile_dir, &summary.cdtitle_path);

    let music_path = resolve_song_asset_path_like_itg(simfile_dir, &summary.music_path)
        .or_else(|| rssp::assets::resolve_music_path_like_itg(simfile_dir, &summary.music_path));

    // Compute audio length (music file duration) in seconds, mirroring ITGmania's
    // m_fMusicLengthSeconds. This intentionally measures the full OGG length,
    // including trailing silence, and is used for displays that call
    // Song:MusicLengthSeconds() in Simply Love.
    //
    // StepMania also applies a safety heuristic: if the decoded music length
    // is suspiciously shorter than the chart's last second (by > 10s), it
    // trusts the chart timing instead. This handles meme files where the
    // audio is a short silent stub but the chart runs for hours.
    let mut music_length_seconds = compute_music_length_seconds(music_path.as_deref());
    let chart_length_seconds = summary.total_length.max(0) as f32;
    if music_length_seconds > 0.0
        && chart_length_seconds > 0.0
        && music_length_seconds < chart_length_seconds - 10.0
    {
        music_length_seconds = chart_length_seconds;
    }

    Ok((
        SongData {
            simfile_path: path.to_path_buf(),
            title: summary.title_str,
            subtitle: summary.subtitle_str,
            translit_title: summary.titletranslit_str,
            translit_subtitle: summary.subtitletranslit_str,
            artist: summary.artist_str,
            banner_path, // Keep original logic for banner
            background_path: background_path_opt,
            background_changes,
            cdtitle_path,
            display_bpm: summary.display_bpm_str,
            offset: summary.offset as f32,
            sample_start: if summary.sample_start > 0.0 {
                Some(summary.sample_start as f32)
            } else {
                None
            },
            sample_length: if summary.sample_length > 0.0 {
                Some(summary.sample_length as f32)
            } else {
                None
            },
            min_bpm: summary.min_bpm,
            max_bpm: summary.max_bpm,
            normalized_bpms: summary.normalized_bpms,
            normalized_stops: summary.normalized_stops,
            normalized_delays: summary.normalized_delays,
            normalized_warps: summary.normalized_warps,
            normalized_speeds: summary.normalized_speeds,
            normalized_scrolls: summary.normalized_scrolls,
            normalized_fakes: summary.normalized_fakes,
            music_path,
            music_length_seconds,
            total_length_seconds: summary.total_length,
            charts,
            cached_precise_last_second: 0.0,
        },
        content_hash,
    ))
>>>>>>> origin/main
}

/// Computes the length of the music file in seconds when the decode layer supports it.
/// Returns 0.0 on failure or if no music path is provided.
fn compute_music_length_seconds(music_path: Option<&Path>) -> f32 {
    let Some(path) = music_path else {
        return 0.0;
    };
    match decode::file_length_seconds(path) {
        Ok(sec) => sec,
        Err(e) => {
            warn!("Failed to compute audio length for {path:?}: {e}");
            0.0
        }
    }
}
