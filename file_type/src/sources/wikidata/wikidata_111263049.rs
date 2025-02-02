use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111263049: FileFormat = FileFormat {
    id: 111_263_049,
    source_type: SourceType::Wikidata,
    name: "ActiveMovie streaming format",
    extensions: &["asf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
