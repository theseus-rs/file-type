use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119406817: FileFormat = FileFormat {
    id: 119_406_817,
    puid: "wikidata/119406817",
    name: "ACT! Database Pointer File",
    extensions: &["pad"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
