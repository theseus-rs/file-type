use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2954147763: FileType = FileType {
    file_format: &FileFormat {
        id: 2_954_147_763,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.bearer-choice-wbxml",
        extensions: &[],
        media_types: &["application/vnd.uplanet.bearer-choice-wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
