use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_603336474: FileFormat = FileFormat {
    id: 603_336_474,
    source_type: SourceType::Linguist,
    name: "KakouneScript",
    extensions: &["kak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
