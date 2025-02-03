use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110442377: FileFormat = FileFormat {
    id: 110_442_377,
    source_type: SourceType::Wikidata,
    name: "Daisy-Dot Font File, version III",
    extensions: &["nlq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
