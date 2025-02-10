use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3008781912: FileType = FileType {
    file_format: &FileFormat {
        id: 3_008_781_912,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.formula-template",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.formula-template"],
        signatures: &[],
        related_formats: &[],
    },
};
