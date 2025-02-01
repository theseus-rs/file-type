use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111190435: FileFormat = FileFormat {
    id: 111_190_435,
    puid: "wikidata/111190435",
    name: "JavaServer Page Document",
    extensions: &["jst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
