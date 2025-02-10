use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3785618288: FileType = FileType {
    file_format: &FileFormat {
        id: 3_785_618_288,
        source_type: SourceType::Iana,
        name: "xmpp+xml",
        extensions: &[],
        media_types: &["application/xmpp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
