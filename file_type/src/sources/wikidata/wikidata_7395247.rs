use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7395247: FileFormat = FileFormat {
    id: 7_395_247,
    source_type: SourceType::Wikidata,
    name: "SYBYL line notation",
    extensions: &["sln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
