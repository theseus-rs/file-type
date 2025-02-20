use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
