use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60413637: FileFormat = FileFormat {
    id: 60_413_637,
    source_type: SourceType::Wikidata,
    name: "INTERLIS Model File, version 2.3",
    extensions: &["ili"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
