use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206085: FileFormat = FileFormat {
    id: 28_206_085,
    puid: "wikidata/28206085",
    name: "TT High Resolution",
    extensions: &["PI7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
