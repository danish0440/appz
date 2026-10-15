//! The Z App Suite: the thirteen apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

use serde::{Deserialize, Serialize};

/// Which generation of the suite an app belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppGroup {
    /// The seven original apps.
    Classic,
    /// The six apps added after them.
    New,
}

impl AppGroup {
    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::New => "New",
        }
    }
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photoz`).
    pub id: &'static str,
    /// Display name (`PhotoZ`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    pub group: AppGroup,
    /// Brand colour (sRGB), for surfaces: getartcraft.com's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (PdfZ shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (PdfZ's MSI said PrintCraft until 0.2.1).
    pub former_names: &'static [&'static str],
}

impl CraftApp {
    /// `com.pintaz.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        format!("com.pintaz.{}", self.id)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's page on GitHub.
    pub fn website(&self) -> String {
        format!("https://github.com/danish0440/{}", self.id)
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    group: AppGroup,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp { id, name, tagline, group, color, ink_light, ink_dark, repo, former_ids: &[], former_names: &[] }
}

/// The Z App Suite, in display order: the seven classic apps, then the six new ones.
pub const SUITE: [CraftApp; 13] = [
    app(
        "wordz",
        "WordZ",
        "Writing and document design",
        AppGroup::New,
        [[0x2e, 0x62, 0xd9], [0x24, 0x4c, 0xa9], [0x77, 0x99, 0xe6]],
        "danish0440/wordz",
    ),
    app(
        "gridz",
        "GridZ",
        "Spreadsheets and data",
        AppGroup::New,
        [[0x1d, 0x9a, 0x5b], [0x17, 0x78, 0x47], [0x6c, 0xbd, 0x94]],
        "danish0440/gridz",
    ),
    app(
        "deckz",
        "DeckZ",
        "Presentations and slide shows",
        AppGroup::New,
        [[0xe3, 0x5d, 0x3a], [0xb1, 0x49, 0x2d], [0xed, 0x96, 0x7f]],
        "danish0440/deckz",
    ),
    app(
        "photoz",
        "PhotoZ",
        "Image editing",
        AppGroup::Classic,
        [[0x0b, 0x8c, 0x9e], [0x09, 0x6d, 0x7b], [0x60, 0xb4, 0xc0]],
        "danish0440/photoz",
    ),
    CraftApp {
        former_ids: &["printcraft"],
        former_names: &["PrintCraft"],
        ..app(
            "pdfz",
            "PdfZ",
            "Reading, organizing and protecting PDFs",
            AppGroup::Classic,
            [[0xd6, 0x3a, 0x3a], [0xa7, 0x2d, 0x2d], [0xe4, 0x7f, 0x7f]],
            "danish0440/pdfz",
        )
    },
    app(
        "filmz",
        "FilmZ",
        "Video editing, color and sound",
        AppGroup::Classic,
        [[0x6a, 0x3f, 0xd0], [0x53, 0x31, 0xa2], [0x9e, 0x82, 0xe0]],
        "danish0440/filmz",
    ),
    app(
        "cadz",
        "CadZ",
        "Computer-aided design and drafting",
        AppGroup::New,
        [[0x3e, 0x5c, 0x6e], [0x30, 0x48, 0x56], [0x82, 0x95, 0xa1]],
        "danish0440/cadz",
    ),
    app(
        "vectorz",
        "VectorZ",
        "Vector illustration",
        AppGroup::Classic,
        [[0xf0, 0x7f, 0x2d], [0xbb, 0x63, 0x23], [0xf5, 0xac, 0x77]],
        "danish0440/vectorz",
    ),
    app(
        "lightz",
        "LightZ",
        "Photo library and raw development",
        AppGroup::Classic,
        [[0x14, 0x96, 0x7f], [0x10, 0x75, 0x63], [0x66, 0xbb, 0xac]],
        "danish0440/lightz",
    ),
    app(
        "effectz",
        "EffectZ",
        "Motion graphics and visual effects",
        AppGroup::Classic,
        [[0x8e, 0x44, 0xad], [0x6f, 0x35, 0x87], [0xb6, 0x85, 0xca]],
        "danish0440/effectz",
    ),
    app(
        "figz",
        "FigZ",
        "Interface design and prototyping",
        AppGroup::New,
        [[0xc2, 0x33, 0x8a], [0x97, 0x28, 0x6c], [0xd7, 0x7a, 0xb3]],
        "danish0440/figz",
    ),
    app(
        "designz",
        "DesignZ",
        "Page layout and publishing",
        AppGroup::Classic,
        [[0xd9, 0x46, 0x6f], [0xa9, 0x37, 0x57], [0xe6, 0x87, 0xa1]],
        "danish0440/designz",
    ),
    app(
        "soundz",
        "SoundZ",
        "Recording, editing and mixing audio",
        AppGroup::New,
        [[0x2f, 0x3e, 0x9e], [0x25, 0x30, 0x7b], [0x78, 0x82, 0xc0]],
        "danish0440/soundz",
    ),
];

/// ArtCraft's website.
pub const WEBSITE_URL: &str = "https://github.com/danish0440/appz";
/// The GitHub account every Z App's repository lives under: the only one the launcher
/// installs from, whatever the release feed says.
pub const GITHUB_OWNER: &str = "danish0440";
/// Source code on GitHub: every Z App's source.
pub const GITHUB_URL: &str = "https://github.com/danish0440";
/// ArtCraft's social accounts: (service, URL). Shown as small icons, less prominent than Discord.
pub const SOCIAL: [(&str, &str); 0] = []; // PINTAZ: no upstream social accounts

/// The community on GitHub.
pub const DISCORD_URL: &str = "https://github.com/danish0440/appz/issues";

/// AppZ itself, described like a Z App so it can update itself with the same
/// machinery (asset names `appz-<ver>-…`, bundle id `com.pintaz.appz`,
/// `AppZ.app`). Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = app(
    "appz",
    "AppZ",
    "Installs and updates the Z App Suite",
    AppGroup::New,
    [[0x0b, 0x0b, 0x0c], [0x0b, 0x0b, 0x0c], [0xf2, 0xf1, 0xee]],
    "danish0440/appz",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "danish0440/appz";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "appz";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_has_seven_classic_and_six_new_apps_with_unique_ids() {
        assert_eq!(SUITE.iter().filter(|a| a.group == AppGroup::Classic).count(), 7);
        assert_eq!(SUITE.iter().filter(|a| a.group == AppGroup::New).count(), 6);
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.repo, format!("storytold/{}", a.id));
            assert_eq!(a.name.to_lowercase(), a.id);
        }
    }

    #[test]
    fn pdfz_also_answers_to_printcraft() {
        let pdf = CraftApp::by_id("pdfz").unwrap();
        assert_eq!(pdf.stems().collect::<Vec<_>>(), ["pdfz", "printcraft"]);
        assert_eq!(pdf.names().collect::<Vec<_>>(), ["PdfZ", "PrintCraft"]);
        assert_eq!(pdf.website(), "https://github.com/danish0440/pdfz");
        assert_eq!(CraftApp::by_id("wordz").unwrap().website(), "https://github.com/danish0440/wordz");
    }
}
