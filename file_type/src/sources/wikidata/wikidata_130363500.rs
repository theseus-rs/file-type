use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130363500: FileFormat = FileFormat {
    id: 130_363_500,
    puid: "wikidata/130363500",
    name: "NCL file",
    extensions: &["ncl"],
    media_types: &["text/ncl"],
    internal_signatures: &[],
    related_formats: &[],
};
