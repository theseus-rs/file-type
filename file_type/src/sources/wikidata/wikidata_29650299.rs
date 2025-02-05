use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650299: FileFormat = FileFormat {
    id: 29_650_299,
    source_type: SourceType::Wikidata,
    name: "PUZ",
    extensions: &["puz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
