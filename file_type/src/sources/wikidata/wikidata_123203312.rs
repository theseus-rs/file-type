use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123203312: FileFormat = FileFormat {
    id: 123_203_312,
    source_type: SourceType::Wikidata,
    name: "TiVo Video File",
    extensions: &["tivo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
