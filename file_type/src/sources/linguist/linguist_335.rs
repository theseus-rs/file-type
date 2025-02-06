use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_335: FileFormat = FileFormat {
    id: 335,
    source_type: SourceType::Linguist,
    name: "SRecode Template",
    extensions: &["srt"],
    media_types: &["text/x-common-lisp"],
    signatures: &[],
    related_formats: &[],
};
