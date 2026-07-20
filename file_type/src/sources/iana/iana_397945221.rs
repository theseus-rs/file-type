use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_397945221: FileType = FileType {
    file_format: &FileFormat {
        id: 397_945_221,
        source_type: SourceType::Iana,
        name: "vnd.hdfgroup.hdf4",
        extensions: &[],
        media_types: &["application/vnd.hdfgroup.hdf4"],
        signatures: &[],
        related_formats: &[],
    },
};
