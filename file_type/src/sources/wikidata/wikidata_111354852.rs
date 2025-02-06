use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111354852: FileFormat = FileFormat {
    id: 111_354_852,
    source_type: SourceType::Wikidata,
    name: "Yamaha Tyros 2 custom voice file",
    extensions: &["tvn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
