use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60413560: FileFormat = FileFormat {
    id: 60_413_560,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Transfer File, version 2.3",
    extensions: &["xtf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
