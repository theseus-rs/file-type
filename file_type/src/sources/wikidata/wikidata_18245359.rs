use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_18245359: FileType = FileType {
    file_format: &FileFormat {
        id: 18_245_359,
        source_type: SourceType::Wikidata,
        name: "Control Panel Applet",
        extensions: &["cpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
