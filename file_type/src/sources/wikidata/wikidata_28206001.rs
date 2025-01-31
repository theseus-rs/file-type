use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206001: FileFormat = FileFormat {
    id: 28_206_001,
    puid: "wikidata/28206001",
    name: "Digital Video Interactive Device-dependent Data",
    extensions: &["i8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
