use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966903: FileFormat = FileFormat {
    id: 27_966_903,
    puid: "wikidata/27966903",
    name: "KSSX",
    extensions: &["kss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
