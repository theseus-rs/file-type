use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_863330050: FileType = FileType {
    file_format: &FileFormat {
        id: 863_330_050,
        source_type: SourceType::Httpd,
        name: "hdf",
        extensions: &["hdf"],
        media_types: &["application/x-hdf"],
        signatures: &[],
        related_formats: &[],
    },
};
