use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757652: FileFormat = FileFormat {
    id: 28_757_652,
    puid: "wikidata/28757652",
    name: "G64",
    extensions: &["g64"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
