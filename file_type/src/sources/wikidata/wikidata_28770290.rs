use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770290: FileFormat = FileFormat {
    id: 28_770_290,
    puid: "wikidata/28770290",
    name: "LSB",
    extensions: &["lsb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
