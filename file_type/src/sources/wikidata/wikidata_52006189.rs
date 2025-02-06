use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52006189: FileFormat = FileFormat {
    id: 52_006_189,
    source_type: SourceType::Wikidata,
    name: "Micrografx Draw, version 4",
    extensions: &["drw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
