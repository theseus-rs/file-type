use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_335: FileType = FileType {
    file_format: &FileFormat {
        id: 335,
        source_type: SourceType::Linguist,
        name: "SRecode Template",
        extensions: &["srt"],
        media_types: &["text/x-common-lisp"],
        signatures: &[],
        related_formats: &[],
    },
};
