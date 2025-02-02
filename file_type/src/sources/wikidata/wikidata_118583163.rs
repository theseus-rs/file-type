use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118583163: FileFormat = FileFormat {
    id: 118_583_163,
    source_type: SourceType::Wikidata,
    name: "Kinetic Project",
    extensions: &["kin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
