use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111417227: FileFormat = FileFormat {
    id: 111_417_227,
    puid: "wikidata/111417227",
    name: "C++ source code file",
    extensions: &["C", "c", "cc", "cpp", "cxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
