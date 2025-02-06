use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650309: FileFormat = FileFormat {
    id: 29_650_309,
    source_type: SourceType::Wikidata,
    name: "PQA",
    extensions: &["pqa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
