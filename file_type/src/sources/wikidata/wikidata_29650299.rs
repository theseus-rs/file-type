use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650299: FileFormat = FileFormat {
    id: 29_650_299,
    source_type: SourceType::Wikidata,
    name: "PUZ",
    extensions: &["puz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
