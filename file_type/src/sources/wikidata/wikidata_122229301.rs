use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122229301: FileType = FileType {
    file_format: &FileFormat {
        id: 122_229_301,
        source_type: SourceType::Wikidata,
        name: "PGP Whole Disk Encryption",
        extensions: &["wde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
