use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131389582: FileFormat = FileFormat {
    id: 131_389_582,
    source_type: SourceType::Wikidata,
    name: "Velocity file format",
    extensions: &["vm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
