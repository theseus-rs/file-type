use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6912474: FileFormat = FileFormat {
    id: 6_912_474,
    source_type: SourceType::Wikidata,
    name: "Mork",
    extensions: &["dat", "mab", "msf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
