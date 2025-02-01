use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206010: FileFormat = FileFormat {
    id: 28_206_010,
    puid: "wikidata/28206010",
    name: "Digital Video Interactive Y Luminance Channel (Compressed 8-bit)",
    extensions: &["cmy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
