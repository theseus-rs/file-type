use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_926165: FileFormat = FileFormat {
    id: 926_165,
    source_type: SourceType::Wikidata,
    name: "Geography Markup Language",
    extensions: &["gml"],
    media_types: &["application/gml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
