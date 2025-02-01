use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206883: FileFormat = FileFormat {
    id: 28_206_883,
    puid: "wikidata/28206883",
    name: "Planetary Data System",
    extensions: &["img", "imq", "lbl", "pds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
