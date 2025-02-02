use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650309: FileFormat = FileFormat {
    id: 29_650_309,
    source_type: SourceType::Wikidata,
    name: "PQA",
    extensions: &["pqa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
