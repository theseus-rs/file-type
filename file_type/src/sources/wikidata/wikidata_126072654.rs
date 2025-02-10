use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126072654: FileType = FileType {
    file_format: &FileFormat {
        id: 126_072_654,
        source_type: SourceType::Wikidata,
        name: "WinFax Sent / Received Document file",
        extensions: &["fxr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
