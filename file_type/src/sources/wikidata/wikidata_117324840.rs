use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117324840: FileFormat = FileFormat {
    id: 117_324_840,
    puid: "wikidata/117324840",
    name: "Function Tree file",
    extensions: &["fp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
