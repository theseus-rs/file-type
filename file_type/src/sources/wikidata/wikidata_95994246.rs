use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95994246: FileFormat = FileFormat {
    id: 95_994_246,
    puid: "wikidata/95994246",
    name: "Agilent Microarray file format",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
