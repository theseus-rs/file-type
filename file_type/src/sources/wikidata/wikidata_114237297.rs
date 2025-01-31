use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114237297: FileFormat = FileFormat {
    id: 114_237_297,
    puid: "wikidata/114237297",
    name: "Visual C++ Project file",
    extensions: &["mak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
