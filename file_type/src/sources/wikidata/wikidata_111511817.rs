use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111511817: FileFormat = FileFormat {
    id: 111_511_817,
    source_type: SourceType::Wikidata,
    name: "Visual Basic Binary Form file",
    extensions: &["frx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
