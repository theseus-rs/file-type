use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111432370: FileFormat = FileFormat {
    id: 111_432_370,
    source_type: SourceType::Wikidata,
    name: "Interface Definition Language File",
    extensions: &["idl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
