use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206343: FileFormat = FileFormat {
    id: 28_206_343,
    puid: "wikidata/28206343",
    name: "ImgStar",
    extensions: &["cpx", "flt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
