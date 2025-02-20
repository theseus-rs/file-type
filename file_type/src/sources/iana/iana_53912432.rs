use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_53912432: FileType = FileType {
    file_format: &FileFormat {
        id: 53_912_432,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.alert-wbxml",
        extensions: &[],
        media_types: &["application/vnd.uplanet.alert-wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
