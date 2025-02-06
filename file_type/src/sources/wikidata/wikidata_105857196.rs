use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857196: FileFormat = FileFormat {
    id: 105_857_196,
    source_type: SourceType::Wikidata,
    name: "Hydrogen Pattern",
    extensions: &["h2pattern"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
