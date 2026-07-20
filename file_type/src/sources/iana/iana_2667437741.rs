use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2667437741: FileType = FileType {
    file_format: &FileFormat {
        id: 2_667_437_741,
        source_type: SourceType::Iana,
        name: "vnd.mohnetic",
        extensions: &[],
        media_types: &["application/vnd.mohnetic"],
        signatures: &[],
        related_formats: &[],
    },
};
