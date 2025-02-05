use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_140848857: FileFormat = FileFormat {
    id: 140_848_857,
    source_type: SourceType::Linguist,
    name: "KiCad Legacy Layout",
    extensions: &["brd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
