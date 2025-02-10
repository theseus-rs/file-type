use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34740349: FileType = FileType {
    file_format: &FileFormat {
        id: 34_740_349,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 1 Comment Data",
        extensions: &["fcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
