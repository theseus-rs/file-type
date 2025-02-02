use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60628005: FileFormat = FileFormat {
    id: 60_628_005,
    source_type: SourceType::Wikidata,
    name: "FoxPro Database, version 2",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
