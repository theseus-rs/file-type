use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967138: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_138,
        source_type: SourceType::Wikidata,
        name: "DigiBooster v1.x module",
        extensions: &["digi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
