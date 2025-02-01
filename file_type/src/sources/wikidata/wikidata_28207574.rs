use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207574: FileFormat = FileFormat {
    id: 28_207_574,
    puid: "wikidata/28207574",
    name: "Zoomify PFF",
    extensions: &["pff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
