use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111417314: FileFormat = FileFormat {
    id: 111_417_314,
    source_type: SourceType::Wikidata,
    name: "Borland Turbo C++ Project File",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
