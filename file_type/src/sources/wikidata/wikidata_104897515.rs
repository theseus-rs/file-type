use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_104897515: FileType = FileType {
    file_format: &FileFormat {
        id: 104_897_515,
        source_type: SourceType::Wikidata,
        name: "Propellerhead Reason ReFill Sound Bank",
        extensions: &["rfl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
