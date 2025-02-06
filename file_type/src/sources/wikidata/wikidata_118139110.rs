use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118139110: FileFormat = FileFormat {
    id: 118_139_110,
    source_type: SourceType::Wikidata,
    name: "Calendar Creator 2.x Event File",
    extensions: &["cee"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
