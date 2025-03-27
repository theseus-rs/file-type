use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133567365: FileType = FileType {
    file_format: &FileFormat {
        id: 133_567_365,
        source_type: SourceType::Wikidata,
        name: "Draw256 VGA Format",
        extensions: &["vga"],
        media_types: &["image/x-draw256-vga"],
        signatures: &[],
        related_formats: &[],
    },
};
