use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121913987: FileFormat = FileFormat {
    id: 121_913_987,
    source_type: SourceType::Wikidata,
    name: "Digital Voice File TRC Codec",
    extensions: &["dvf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
