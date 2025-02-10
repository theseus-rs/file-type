use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_154848389: FileType = FileType {
    file_format: &FileFormat {
        id: 154_848_389,
        source_type: SourceType::Iana,
        name: "vnd.hgl",
        extensions: &[],
        media_types: &["text/vnd.hgl"],
        signatures: &[],
        related_formats: &[],
    },
};
