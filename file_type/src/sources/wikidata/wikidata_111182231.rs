use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111182231: FileFormat = FileFormat {
    id: 111_182_231,
    puid: "wikidata/111182231",
    name: "ActionScript Communication File",
    extensions: &["asc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
