use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25101636: FileFormat = FileFormat {
    id: 25_101_636,
    puid: "wikidata/25101636",
    name: "IVUE",
    extensions: &["ivue"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
