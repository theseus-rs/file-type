use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117842873: FileFormat = FileFormat {
    id: 117_842_873,
    puid: "wikidata/117842873",
    name: "EDMICS 6",
    extensions: &["ed6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
