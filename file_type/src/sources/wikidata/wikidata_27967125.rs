use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967125: FileFormat = FileFormat {
    id: 27_967_125,
    puid: "wikidata/27967125",
    name: "CMC",
    extensions: &["cmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
