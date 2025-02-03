use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_258778: FileFormat = FileFormat {
    id: 258_778,
    source_type: SourceType::Wikidata,
    name: "Extensible Application Markup Language",
    extensions: &["xaml"],
    media_types: &["application/xaml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
