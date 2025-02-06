use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72725061: FileFormat = FileFormat {
    id: 72_725_061,
    source_type: SourceType::Wikidata,
    name: "NATO Secondary Imagery Format",
    extensions: &["nsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
