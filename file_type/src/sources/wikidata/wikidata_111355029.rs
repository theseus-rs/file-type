use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355029: FileFormat = FileFormat {
    id: 111_355_029,
    puid: "wikidata/111355029",
    name: "Unreal Tournament audio package",
    extensions: &["uax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
