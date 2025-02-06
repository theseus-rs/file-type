use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100323905: FileFormat = FileFormat {
    id: 100_323_905,
    source_type: SourceType::Wikidata,
    name: "PFS:Write Document",
    extensions: &["pfs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
