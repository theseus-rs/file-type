use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_258778: FileFormat = FileFormat {
    id: 258_778,
    puid: "wikidata/258778",
    name: "Extensible Application Markup Language",
    extensions: &["xaml"],
    media_types: &["application/xaml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
