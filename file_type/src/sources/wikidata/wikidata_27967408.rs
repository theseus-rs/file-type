use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967408: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_408,
        source_type: SourceType::Wikidata,
        name: "Codisk Audio File",
        extensions: &["caf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
