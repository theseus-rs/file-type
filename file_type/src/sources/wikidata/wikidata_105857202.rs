use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857202: FileFormat = FileFormat {
    id: 105_857_202,
    source_type: SourceType::Wikidata,
    name: "Hydrogen song",
    extensions: &["h2song"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
