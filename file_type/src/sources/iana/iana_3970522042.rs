use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3970522042: FileType = FileType {
    file_format: &FileFormat {
        id: 3_970_522_042,
        source_type: SourceType::Iana,
        name: "vnd.hdfgroup.hdf5",
        extensions: &[],
        media_types: &["application/vnd.hdfgroup.hdf5"],
        signatures: &[],
        related_formats: &[],
    },
};
