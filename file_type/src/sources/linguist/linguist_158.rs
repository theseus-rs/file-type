use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_158: FileFormat = FileFormat {
    id: 158,
    source_type: SourceType::Linguist,
    name: "Haxe",
    extensions: &["hx", "hxsl"],
    media_types: &["text/x-haxe"],
    internal_signatures: &[],
    related_formats: &[],
};
