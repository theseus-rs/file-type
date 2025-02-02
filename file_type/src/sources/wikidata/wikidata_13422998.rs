use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_13422998: FileFormat = FileFormat {
    id: 13_422_998,
    source_type: SourceType::Wikidata,
    name: "HTTP Archive format",
    extensions: &["har"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
