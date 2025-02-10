use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122305824: FileType = FileType {
    file_format: &FileFormat {
        id: 122_305_824,
        source_type: SourceType::Wikidata,
        name: "PressRelease Message",
        extensions: &["prm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
