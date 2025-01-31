use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117842812: FileFormat = FileFormat {
    id: 117_842_812,
    puid: "wikidata/117842812",
    name: "EDMICS 5",
    extensions: &["ed5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
