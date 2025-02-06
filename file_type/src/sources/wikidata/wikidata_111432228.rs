use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111432228: FileFormat = FileFormat {
    id: 111_432_228,
    source_type: SourceType::Wikidata,
    name: "HTTP File Server Template",
    extensions: &["tpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
