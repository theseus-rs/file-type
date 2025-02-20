use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3762653956: FileType = FileType {
    file_format: &FileFormat {
        id: 3_762_653_956,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.text-master-template",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.text-master-template"],
        signatures: &[],
        related_formats: &[],
    },
};
