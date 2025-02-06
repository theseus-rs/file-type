use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7395247: FileFormat = FileFormat {
    id: 7_395_247,
    source_type: SourceType::Wikidata,
    name: "SYBYL line notation",
    extensions: &["sln"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
