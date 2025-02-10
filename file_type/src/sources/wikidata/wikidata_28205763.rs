use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205763: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_763,
        source_type: SourceType::Wikidata,
        name: "Binary Information File",
        extensions: &["bif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
