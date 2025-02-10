use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111355515: FileType = FileType {
    file_format: &FileFormat {
        id: 111_355_515,
        source_type: SourceType::Wikidata,
        name: "Talking Technology Incorporated file",
        extensions: &["vox"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
