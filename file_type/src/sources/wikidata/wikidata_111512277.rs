use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111512277: FileType = FileType {
    file_format: &FileFormat {
        id: 111_512_277,
        source_type: SourceType::Wikidata,
        name: "ASEG-GDF2 Description file",
        extensions: &["des"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
