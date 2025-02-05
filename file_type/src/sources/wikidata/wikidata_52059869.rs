use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52059869: FileFormat = FileFormat {
    id: 52_059_869,
    source_type: SourceType::Wikidata,
    name: "Micrografx Designer format, version 3.1",
    extensions: &["drw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
