use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967128: FileFormat = FileFormat {
    id: 27_967_128,
    puid: "wikidata/27967128",
    name: "DMC",
    extensions: &["dmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
