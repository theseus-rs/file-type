use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206024: FileFormat = FileFormat {
    id: 28_206_024,
    puid: "wikidata/28206024",
    name: "Digital Video Interactive Device-dependent Data (Compressed 16-bit)",
    extensions: &["c16"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
