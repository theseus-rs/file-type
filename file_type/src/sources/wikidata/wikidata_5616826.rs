use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5616826: FileFormat = FileFormat {
    id: 5_616_826,
    puid: "wikidata/5616826",
    name: "ChordPro",
    extensions: &["cho", "chopro", "crd", "pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
