use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207000: FileFormat = FileFormat {
    id: 28_207_000,
    puid: "wikidata/28207000",
    name: "Picture Packer",
    extensions: &["pp1", "pp2", "pp3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
