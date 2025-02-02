use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131395745: FileFormat = FileFormat {
    id: 131_395_745,
    source_type: SourceType::Wikidata,
    name: "VGL source code file",
    extensions: &["rpf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
