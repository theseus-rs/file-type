use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112819749: FileType = FileType {
    file_format: &FileFormat {
        id: 112_819_749,
        source_type: SourceType::Wikidata,
        name: "Detailer 3D File",
        extensions: &["vdu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
