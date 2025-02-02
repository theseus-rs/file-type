use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72725061: FileFormat = FileFormat {
    id: 72_725_061,
    source_type: SourceType::Wikidata,
    name: "NATO Secondary Imagery Format",
    extensions: &["nsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
