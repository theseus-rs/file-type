use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122408622: FileType = FileType {
    file_format: &FileFormat {
        id: 122_408_622,
        source_type: SourceType::Wikidata,
        name: "68K Symbol File",
        extensions: &["sym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
