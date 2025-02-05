use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_774635084: FileFormat = FileFormat {
    id: 774_635_084,
    source_type: SourceType::Linguist,
    name: "Jest Snapshot",
    extensions: &["snap"],
    media_types: &["application/javascript"],
    signatures: &[],
    related_formats: &[],
};
