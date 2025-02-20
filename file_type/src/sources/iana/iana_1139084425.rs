use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1139084425: FileType = FileType {
    file_format: &FileFormat {
        id: 1_139_084_425,
        source_type: SourceType::Iana,
        name: "vnd.trolltech.linguist",
        extensions: &[],
        media_types: &["text/vnd.trolltech.linguist"],
        signatures: &[],
        related_formats: &[],
    },
};
