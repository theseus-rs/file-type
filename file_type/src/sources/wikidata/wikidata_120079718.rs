use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120079718: FileFormat = FileFormat {
    id: 120_079_718,
    puid: "wikidata/120079718",
    name: "Matisse file",
    extensions: &["mat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
