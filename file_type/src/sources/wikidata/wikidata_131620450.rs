use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131620450: FileFormat = FileFormat {
    id: 131_620_450,
    puid: "wikidata/131620450",
    name: "Ansys Fluent Data file format",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
