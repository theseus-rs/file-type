use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125134441: FileType = FileType {
    file_format: &FileFormat {
        id: 125_134_441,
        source_type: SourceType::Wikidata,
        name: "YAM Folders",
        extensions: &["folders"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
