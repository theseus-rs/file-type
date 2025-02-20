use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112661245: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_245,
        source_type: SourceType::Wikidata,
        name: "Autodesk Inventor Assembly file format",
        extensions: &["iam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
