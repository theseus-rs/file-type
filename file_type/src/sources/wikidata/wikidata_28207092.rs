use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207092: FileFormat = FileFormat {
    id: 28_207_092,
    puid: "wikidata/28207092",
    name: "PrintMaster Index file",
    extensions: &["sdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
