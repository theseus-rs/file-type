use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47520869: FileFormat = FileFormat {
    id: 47_520_869,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version 12",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
