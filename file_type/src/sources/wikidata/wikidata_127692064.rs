use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127692064: FileFormat = FileFormat {
    id: 127_692_064,
    puid: "wikidata/127692064",
    name: "freefem format",
    extensions: &["msh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
