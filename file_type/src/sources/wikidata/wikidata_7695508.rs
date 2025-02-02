use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7695508: FileFormat = FileFormat {
    id: 7_695_508,
    source_type: SourceType::Wikidata,
    name: "Tektronix extended HEX",
    extensions: &["tek"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
