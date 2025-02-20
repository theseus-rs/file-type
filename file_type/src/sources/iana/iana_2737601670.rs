use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2737601670: FileType = FileType {
    file_format: &FileFormat {
        id: 2_737_601_670,
        source_type: SourceType::Iana,
        name: "vnd.afpc.cmoca-cmresource",
        extensions: &[],
        media_types: &["application/vnd.afpc.cmoca-cmresource"],
        signatures: &[],
        related_formats: &[],
    },
};
