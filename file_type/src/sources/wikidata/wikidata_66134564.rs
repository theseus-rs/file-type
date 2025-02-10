use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66134564: FileType = FileType {
    file_format: &FileFormat {
        id: 66_134_564,
        source_type: SourceType::Wikidata,
        name: "Photoshop DCS 1.0",
        extensions: &["eps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
