use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135487499: FileType = FileType {
    file_format: &FileFormat {
        id: 135_487_499,
        source_type: SourceType::Wikidata,
        name: "M-Tx file format",
        extensions: &["mtx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
