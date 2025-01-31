use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51093528: FileFormat = FileFormat {
    id: 51_093_528,
    puid: "wikidata/51093528",
    name: "CorelDraw Pattern",
    extensions: &["pat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
