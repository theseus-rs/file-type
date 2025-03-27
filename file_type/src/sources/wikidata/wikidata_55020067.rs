use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55020067: FileType = FileType {
    file_format: &FileFormat {
        id: 55_020_067,
        source_type: SourceType::Wikidata,
        name: "Reflectance Transformation Imaging file format",
        extensions: &["rti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
