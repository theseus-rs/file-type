use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76515206: FileFormat = FileFormat {
    id: 76_515_206,
    puid: "wikidata/76515206",
    name: "Lotus 123 SQZ! compressed Worksheet",
    extensions: &["wk!"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
