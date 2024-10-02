//! Application state.

use norad::glyph::{Contour, ContourPoint, Glyph, GlyphName, PointType};
use norad::{FontInfo, Ufo};

// Trying to change the default convention to 1024. :-)
const DEFAULT_UNITS_PER_EM: f64 = 1024.;
const DEFAULT_PREVIEW_FONT_SIZE: f64 = 64.0;

/// The top level data structure.
/// Currently this just wraps `Workspace`.
/// In the future multiple workspaces will be possible.
#[derive(Clone, Data, Default, Lens)]
pub struct AppState {
    pub workspace: Workspace,
}

