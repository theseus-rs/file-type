use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110086227: FileFormat = FileFormat {
    id: 110_086_227,
    puid: "wikidata/110086227",
    name: "NTI JewelCase Maker format",
    extensions: &["jwc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
