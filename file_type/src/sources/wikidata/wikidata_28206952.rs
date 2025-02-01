use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206952: FileFormat = FileFormat {
    id: 28_206_952,
    puid: "wikidata/28206952",
    name: "PhotoDeluxe",
    extensions: &["pdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
