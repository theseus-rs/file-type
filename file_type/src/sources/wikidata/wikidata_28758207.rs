use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28758207: FileFormat = FileFormat {
    id: 28_758_207,
    puid: "wikidata/28758207",
    name: "Adaptive Prediction Trees",
    extensions: &["apt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
