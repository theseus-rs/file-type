use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
