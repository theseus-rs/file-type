use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118583163: FileFormat = FileFormat {
    id: 118_583_163,
    source_type: SourceType::Wikidata,
    name: "Kinetic Project",
    extensions: &["kin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
