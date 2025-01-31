use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206013: FileFormat = FileFormat {
    id: 28_206_013,
    puid: "wikidata/28206013",
    name: "Digital Video Interactive I Color Channel (Compressed 8-bit)",
    extensions: &["cmi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
