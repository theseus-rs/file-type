use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60413637: FileFormat = FileFormat {
    id: 60_413_637,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Model File, version 2.3",
    extensions: &["ili"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
