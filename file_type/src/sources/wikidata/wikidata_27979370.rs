use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979370: FileFormat = FileFormat {
    id: 27_979_370,
    puid: "wikidata/27979370",
    name: "EBU STL",
    extensions: &["stl"],
    media_types: &["application/x-ebu-stl"],
    internal_signatures: &[],
    related_formats: &[],
};
