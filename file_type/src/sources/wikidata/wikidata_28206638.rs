use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206638: FileFormat = FileFormat {
    id: 28_206_638,
    puid: "wikidata/28206638",
    name: "MTV ray tracer bitmap",
    extensions: &["mtv", "pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
