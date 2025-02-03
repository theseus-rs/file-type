use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115037504: FileFormat = FileFormat {
    id: 115_037_504,
    source_type: SourceType::Wikidata,
    name: "Extensible Markup Language 1.1",
    extensions: &["xml"],
    media_types: &["application/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
