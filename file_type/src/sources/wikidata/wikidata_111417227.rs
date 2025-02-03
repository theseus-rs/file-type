use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111417227: FileFormat = FileFormat {
    id: 111_417_227,
    source_type: SourceType::Wikidata,
    name: "C++ source code file",
    extensions: &["C", "c", "cc", "cpp", "cxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
