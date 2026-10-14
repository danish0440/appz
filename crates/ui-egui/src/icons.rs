//! Bundled images: each Z App's icon and the launcher's own.

use egui::ImageSource;

/// The 128 px icon of a Z App.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "photoz" => egui::include_image!("../../../assets/app-icons/photoz.png"),
        "vectorz" => egui::include_image!("../../../assets/app-icons/vectorz.png"),
        "filmz" => egui::include_image!("../../../assets/app-icons/filmz.png"),
        "lightz" => egui::include_image!("../../../assets/app-icons/lightz.png"),
        "pdfz" => egui::include_image!("../../../assets/app-icons/pdfz.png"),
        "effectz" => egui::include_image!("../../../assets/app-icons/effectz.png"),
        "designz" => egui::include_image!("../../../assets/app-icons/designz.png"),
        "wordz" => egui::include_image!("../../../assets/app-icons/wordz.png"),
        "deckz" => egui::include_image!("../../../assets/app-icons/deckz.png"),
        "gridz" => egui::include_image!("../../../assets/app-icons/gridz.png"),
        "cadz" => egui::include_image!("../../../assets/app-icons/cadz.png"),
        "soundz" => egui::include_image!("../../../assets/app-icons/soundz.png"),
        "figz" => egui::include_image!("../../../assets/app-icons/figz.png"),
        _ => return None,
    })
}

/// The ArtCraft wordmark, as supplied: dark ink for light backgrounds, white for dark ones.
/// An ArtCraft trademark, not covered by the MIT/Apache license (`docs/brand/LICENSE-brand.txt`).
pub fn artcraft_logo(dark_background: bool) -> ImageSource<'static> {
    if dark_background {
        egui::include_image!("../../../assets/z-brand/logo-white.svg")
    } else {
        egui::include_image!("../../../assets/z-brand/logo.svg")
    }
}

/// The ArtCraft wordmark's width / height (its SVG view box: 1038.05 × 108).
pub const ARTCRAFT_LOGO_ASPECT: f32 = 1038.05 / 108.0;

/// A social service's logo (Simple Icons, white, for tinting), by its name in `catalog::SOCIAL`.
pub fn social(service: &str) -> Option<ImageSource<'static>> {
    Some(match service {
        "X" => egui::include_image!("../../../assets/social/x.svg"),
        "YouTube" => egui::include_image!("../../../assets/social/youtube.svg"),
        "TikTok" => egui::include_image!("../../../assets/social/tiktok.svg"),
        "Instagram" => egui::include_image!("../../../assets/social/instagram.svg"),
        "LinkedIn" => egui::include_image!("../../../assets/social/linkedin.svg"),
        _ => return None,
    })
}

/// The ArtCraft "A" mark in blue, as supplied: the launcher's app icon and getartcraft.com's favicon.
pub fn artcraft_mark() -> ImageSource<'static> {
    egui::include_image!("../../../assets/z-brand/mark.svg")
}
