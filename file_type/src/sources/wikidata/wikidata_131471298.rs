use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131471298: FileFormat = FileFormat {
    id: 131_471_298,
    puid: "wikidata/131471298",
    name: "MGH file format",
    extensions: &["mgh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
