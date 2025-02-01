use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979367: FileFormat = FileFormat {
    id: 27_979_367,
    puid: "wikidata/27979367",
    name: "ReScene",
    extensions: &["srr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
