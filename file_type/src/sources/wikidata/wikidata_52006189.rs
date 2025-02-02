use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52006189: FileFormat = FileFormat {
    id: 52_006_189,
    source_type: SourceType::Wikidata,
    name: "Micrografx Draw, version 4",
    extensions: &["drw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
