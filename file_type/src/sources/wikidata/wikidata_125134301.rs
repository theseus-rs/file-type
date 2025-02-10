use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125134301: FileType = FileType {
    file_format: &FileFormat {
        id: 125_134_301,
        source_type: SourceType::Wikidata,
        name: "YAM Addressbook",
        extensions: &["addressbook"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
