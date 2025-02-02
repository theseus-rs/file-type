use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111354852: FileFormat = FileFormat {
    id: 111_354_852,
    source_type: SourceType::Wikidata,
    name: "Yamaha Tyros 2 custom voice file",
    extensions: &["tvn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
