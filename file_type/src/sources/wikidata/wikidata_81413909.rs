use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81413909: FileFormat = FileFormat {
    id: 81_413_909,
    puid: "wikidata/81413909",
    name: "Macromedia Director Shockwave Cast",
    extensions: &["cct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
