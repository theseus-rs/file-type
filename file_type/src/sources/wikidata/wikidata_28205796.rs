use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205796: FileFormat = FileFormat {
    id: 28_205_796,
    source_type: SourceType::Wikidata,
    name: "Master of Orion saved game",
    extensions: &["gam"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
