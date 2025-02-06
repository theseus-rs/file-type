use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_926165: FileFormat = FileFormat {
    id: 926_165,
    source_type: SourceType::Wikidata,
    name: "Geography Markup Language",
    extensions: &["gml"],
    media_types: &["application/gml+xml"],
    signatures: &[],
    related_formats: &[],
};
