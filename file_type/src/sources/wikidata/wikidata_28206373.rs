use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206373: FileFormat = FileFormat {
    id: 28_206_373,
    puid: "wikidata/28206373",
    name: "Interleaf image",
    extensions: &["iimg", "img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
