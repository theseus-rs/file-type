use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111578627: FileType = FileType {
    file_format: &FileFormat {
        id: 111_578_627,
        source_type: SourceType::Wikidata,
        name: "Z Print Build File",
        extensions: &["zbd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
