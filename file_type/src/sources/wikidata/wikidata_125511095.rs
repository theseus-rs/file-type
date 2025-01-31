use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125511095: FileFormat = FileFormat {
    id: 125_511_095,
    puid: "wikidata/125511095",
    name: "Zoner Photo Studio file",
    extensions: &["zps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
