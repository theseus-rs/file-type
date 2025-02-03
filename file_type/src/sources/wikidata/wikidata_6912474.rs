use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6912474: FileFormat = FileFormat {
    id: 6_912_474,
    source_type: SourceType::Wikidata,
    name: "Mork",
    extensions: &["dat", "mab", "msf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
