use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_451700185: FileType = FileType {
    file_format: &FileFormat {
        id: 451_700_185,
        source_type: SourceType::Linguist,
        name: "AIDL",
        extensions: &["aidl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
