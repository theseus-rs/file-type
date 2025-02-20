use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_883060521: FileType = FileType {
    file_format: &FileFormat {
        id: 883_060_521,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.interworking-data",
        extensions: &[],
        media_types: &["application/vnd.3gpp.interworking-data"],
        signatures: &[],
        related_formats: &[],
    },
};
