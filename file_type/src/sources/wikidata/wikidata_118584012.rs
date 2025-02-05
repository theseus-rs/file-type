use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118584012: FileFormat = FileFormat {
    id: 118_584_012,
    source_type: SourceType::Wikidata,
    name: "Cakewalk Template",
    extensions: &["cwt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
