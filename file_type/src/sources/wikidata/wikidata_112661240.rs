use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112661240: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_240,
        source_type: SourceType::Wikidata,
        name: "Autodesk Inventor Part file format",
        extensions: &["ipt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
