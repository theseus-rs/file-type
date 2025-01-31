use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131620359: FileFormat = FileFormat {
    id: 131_620_359,
    puid: "wikidata/131620359",
    name: "Ansys Fluent file format",
    extensions: &["cas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
