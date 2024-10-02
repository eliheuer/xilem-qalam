//! Application state.

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use norad::glyph::{Contour, ContourPoint, Glyph, GlyphName, PointType};
use norad::{FontInfo, Ufo};

// Trying to change the default convention to 1024. :-)
const DEFAULT_UNITS_PER_EM: f64 = 1024.;
const DEFAULT_PREVIEW_FONT_SIZE: f64 = 64.0;

/// The top level data structure.
/// Currently this just wraps `Workspace`.
/// In the future multiple workspaces will be possible.
#[derive(Clone, Default)]
pub struct AppState {
    pub workspace: Workspace,
}

/// A workspace is a single font, corresponding to a UFO file on disk.
#[derive(Clone, Lens, Data, Default)]
pub struct Workspace {
    pub font: Arc<FontObject>,
    /// The currently selected glyph (in the main glyph list) if any.
    //TODO: allow multiple selections
    pub selected: Option<GlyphName>,
    /// glyphs that are already open in an editor window
    pub open_glyphs: Arc<HashMap<GlyphName, WindowId>>,
    pub sessions: Arc<HashMap<SessionId, Arc<EditSession>>>,
    pub(crate) previews: Arc<HashMap<SessionId, PreviewSession>>,
    session_map: Arc<HashMap<GlyphName, SessionId>>,
    // really just a store of the fully resolved Beziers of all glyphs.
    cache: Arc<BezCache>,
    pub info: SimpleFontInfo,
}


