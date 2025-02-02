use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111432228: FileFormat = FileFormat {
    id: 111_432_228,
    source_type: SourceType::Wikidata,
    name: "HTTP File Server Template",
    extensions: &["tpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
