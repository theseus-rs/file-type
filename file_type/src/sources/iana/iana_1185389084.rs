use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1185389084: FileType = FileType {
    file_format: &FileFormat {
        id: 1_185_389_084,
        source_type: SourceType::Iana,
        name: "vnd.spotfire.dxp",
        extensions: &[],
        media_types: &["application/vnd.spotfire.dxp"],
        signatures: &[],
        related_formats: &[],
    },
};
