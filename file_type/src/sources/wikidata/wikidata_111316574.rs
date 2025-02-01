use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111316574: FileFormat = FileFormat {
    id: 111_316_574,
    puid: "wikidata/111316574",
    name: "Sample Cell II PC instrument",
    extensions: &["ins"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
