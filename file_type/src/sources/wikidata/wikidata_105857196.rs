use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857196: FileFormat = FileFormat {
    id: 105_857_196,
    source_type: SourceType::Wikidata,
    name: "Hydrogen Pattern",
    extensions: &["h2pattern"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
