use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_389: FileFormat = FileFormat {
    id: 389,
    source_type: SourceType::Linguist,
    name: "Visual Basic .NET",
    extensions: &["vb", "vbhtml"],
    media_types: &["text/x-vb"],
    internal_signatures: &[],
    related_formats: &[],
};
