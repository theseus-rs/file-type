use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3846578976: FileType = FileType {
    file_format: &FileFormat {
        id: 3_846_578_976,
        source_type: SourceType::Iana,
        name: "vnd.gov.sk.xmldatacontainer+xml",
        extensions: &[],
        media_types: &["application/vnd.gov.sk.xmldatacontainer+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
