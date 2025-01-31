use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206349: FileFormat = FileFormat {
    id: 28_206_349,
    puid: "wikidata/28206349",
    name: "GEOS Convert",
    extensions: &["cvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
