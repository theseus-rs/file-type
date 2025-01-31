use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206017: FileFormat = FileFormat {
    id: 28_206_017,
    puid: "wikidata/28206017",
    name: "Digital Video Interactive Q Color Channel (Compressed 8-bit)",
    extensions: &["cmq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
