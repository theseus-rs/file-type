use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_914318960: FileType = FileType {
    file_format: &FileFormat {
        id: 914_318_960,
        source_type: SourceType::Linguist,
        name: "JavaScript+ERB",
        extensions: &["js.erb"],
        media_types: &["application/javascript"],
        signatures: &[],
        related_formats: &[],
    },
};
