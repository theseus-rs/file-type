use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206159: FileFormat = FileFormat {
    id: 28_206_159,
    puid: "wikidata/28206159",
    name: "G9B",
    extensions: &["g9b"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
