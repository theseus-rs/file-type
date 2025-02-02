use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118139110: FileFormat = FileFormat {
    id: 118_139_110,
    source_type: SourceType::Wikidata,
    name: "Calendar Creator 2.x Event File",
    extensions: &["cee"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
