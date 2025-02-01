use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206535: FileFormat = FileFormat {
    id: 28_206_535,
    puid: "wikidata/28206535",
    name: "Magick Persistent Cache (.mpc file)",
    extensions: &["mpc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
