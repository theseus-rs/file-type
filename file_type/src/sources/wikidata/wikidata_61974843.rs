use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61974843: FileType = FileType {
    file_format: &FileFormat {
        id: 61_974_843,
        source_type: SourceType::Wikidata,
        name: "FoxPro Compound Index File",
        extensions: &["cdx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
