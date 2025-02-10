use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122168550: FileType = FileType {
    file_format: &FileFormat {
        id: 122_168_550,
        source_type: SourceType::Wikidata,
        name: "Proactive Password Auditor Project",
        extensions: &["hdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
