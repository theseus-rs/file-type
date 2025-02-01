use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7886963: FileFormat = FileFormat {
    id: 7_886_963,
    puid: "wikidata/7886963",
    name: "Uniqueness Database File",
    extensions: &["udf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
