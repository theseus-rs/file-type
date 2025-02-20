use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_104600905: FileType = FileType {
    file_format: &FileFormat {
        id: 104_600_905,
        source_type: SourceType::Wikidata,
        name: "KDBX",
        extensions: &["kdbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
