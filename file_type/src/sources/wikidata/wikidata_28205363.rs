use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205363: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_363,
        source_type: SourceType::Wikidata,
        name: "KDC (P-Series)",
        extensions: &["kdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
