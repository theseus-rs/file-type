use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133502807: FileType = FileType {
    file_format: &FileFormat {
        id: 133_502_807,
        source_type: SourceType::Wikidata,
        name: "Blazing Paddles file",
        extensions: &["pi", "wnd"],
        media_types: &["image/x-blazing-paddles"],
        signatures: &[],
        related_formats: &[],
    },
};
