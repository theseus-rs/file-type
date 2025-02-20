use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73504589: FileType = FileType {
    file_format: &FileFormat {
        id: 73_504_589,
        source_type: SourceType::Wikidata,
        name: "CorelFlow",
        extensions: &["cfl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
