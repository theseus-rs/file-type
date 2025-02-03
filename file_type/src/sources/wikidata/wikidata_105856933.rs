use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856933: FileFormat = FileFormat {
    id: 105_856_933,
    source_type: SourceType::Wikidata,
    name: "NETGEN Constructive Solid Geometry format (with rem)",
    extensions: &["geo"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
