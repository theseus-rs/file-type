use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967433: FileFormat = FileFormat {
    id: 27_967_433,
    puid: "wikidata/27967433",
    name: "Bink Video, version 1",
    extensions: &["bik"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
