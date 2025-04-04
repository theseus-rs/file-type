use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2495353376: FileType = FileType {
    file_format: &FileFormat {
        id: 2_495_353_376,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.text-master",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.text-master"],
        signatures: &[],
        related_formats: &[],
    },
};
