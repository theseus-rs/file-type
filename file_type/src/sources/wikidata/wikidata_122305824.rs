use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
