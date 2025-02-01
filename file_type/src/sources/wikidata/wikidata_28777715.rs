use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28777715: FileFormat = FileFormat {
    id: 28_777_715,
    puid: "wikidata/28777715",
    name: "NSIS",
    extensions: &["exe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
