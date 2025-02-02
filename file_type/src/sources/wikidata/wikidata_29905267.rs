use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905267: FileFormat = FileFormat {
    id: 29_905_267,
    source_type: SourceType::Wikidata,
    name: "Self-Extracting Archive",
    extensions: &["sea"],
    media_types: &["application/x-sea"],
    internal_signatures: &[],
    related_formats: &[],
};
