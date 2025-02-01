use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979410: FileFormat = FileFormat {
    id: 27_979_410,
    puid: "wikidata/27979410",
    name: "Binary Text",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
