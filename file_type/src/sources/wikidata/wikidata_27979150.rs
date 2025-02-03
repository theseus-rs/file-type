use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979150: FileFormat = FileFormat {
    id: 27_979_150,
    source_type: SourceType::Wikidata,
    name: "AN2",
    extensions: &["an2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
