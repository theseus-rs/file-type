use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_972985342: FileType = FileType {
    file_format: &FileFormat {
        id: 972_985_342,
        source_type: SourceType::Iana,
        name: "vnd.dvb.ipdcesgaccess2",
        extensions: &[],
        media_types: &["application/vnd.dvb.ipdcesgaccess2"],
        signatures: &[],
        related_formats: &[],
    },
};
