use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111511817: FileFormat = FileFormat {
    id: 111_511_817,
    source_type: SourceType::Wikidata,
    name: "Visual Basic Binary Form file",
    extensions: &["frx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
