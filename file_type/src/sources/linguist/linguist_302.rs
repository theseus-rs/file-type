use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_302: FileType = FileType {
    file_format: &FileFormat {
        id: 302,
        source_type: SourceType::Linguist,
        name: "PureScript",
        extensions: &["purs"],
        media_types: &["text/x-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
