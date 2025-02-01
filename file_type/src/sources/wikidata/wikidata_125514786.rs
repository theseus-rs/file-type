use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125514786: FileFormat = FileFormat {
    id: 125_514_786,
    puid: "wikidata/125514786",
    name: "Hasselblad RAW Image",
    extensions: &["fff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
