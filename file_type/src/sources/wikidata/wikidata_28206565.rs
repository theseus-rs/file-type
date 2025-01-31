use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206565: FileFormat = FileFormat {
    id: 28_206_565,
    puid: "wikidata/28206565",
    name: "MicroDesign Area",
    extensions: &["mda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
