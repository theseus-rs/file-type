use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130742105: FileType = FileType {
    file_format: &FileFormat {
        id: 130_742_105,
        source_type: SourceType::Wikidata,
        name: "Scaml markup file",
        extensions: &["scaml"],
        media_types: &["text/x-scaml"],
        signatures: &[],
        related_formats: &[],
    },
};
