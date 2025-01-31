use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125040222: FileFormat = FileFormat {
    id: 125_040_222,
    puid: "wikidata/125040222",
    name: "Syntorial file",
    extensions: &["syn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
