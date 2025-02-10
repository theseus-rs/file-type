use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2166834726: FileType = FileType {
    file_format: &FileFormat {
        id: 2_166_834_726,
        source_type: SourceType::Iana,
        name: "sip",
        extensions: &[],
        media_types: &["message/sip"],
        signatures: &[],
        related_formats: &[],
    },
};
