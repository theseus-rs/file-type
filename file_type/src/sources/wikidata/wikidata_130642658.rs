use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130642658: FileType = FileType {
    file_format: &FileFormat {
        id: 130_642_658,
        source_type: SourceType::Wikidata,
        name: "Robot Framework file format",
        extensions: &["robot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
