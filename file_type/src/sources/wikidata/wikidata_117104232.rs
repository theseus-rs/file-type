use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117104232: FileFormat = FileFormat {
    id: 117_104_232,
    puid: "wikidata/117104232",
    name: "Picture it! Publishing File",
    extensions: &["php"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
