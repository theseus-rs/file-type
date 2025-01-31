use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130280165: FileFormat = FileFormat {
    id: 130_280_165,
    puid: "wikidata/130280165",
    name: "Mask file format",
    extensions: &["mask"],
    media_types: &["text/x-mask"],
    internal_signatures: &[],
    related_formats: &[],
};
