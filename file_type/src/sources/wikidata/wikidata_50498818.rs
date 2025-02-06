use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50498818: FileFormat = FileFormat {
    id: 50_498_818,
    source_type: SourceType::Wikidata,
    name: "Geography Markup Language, version 3.2",
    extensions: &["gml"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
