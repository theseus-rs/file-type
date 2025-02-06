use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_474112: FileFormat = FileFormat {
    id: 474_112,
    source_type: SourceType::Wikidata,
    name: "JHTML",
    extensions: &["jhtml"],
    media_types: &["java-internal/java-html"],
    signatures: &[],
    related_formats: &[],
};
