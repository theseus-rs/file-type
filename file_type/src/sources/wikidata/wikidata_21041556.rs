use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21041556: FileFormat = FileFormat {
    id: 21_041_556,
    source_type: SourceType::Wikidata,
    name: "Music Editor format",
    extensions: &["med"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
