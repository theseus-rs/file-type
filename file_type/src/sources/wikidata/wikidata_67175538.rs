use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67175538: FileFormat = FileFormat {
    id: 67_175_538,
    source_type: SourceType::Wikidata,
    name: "Nero Digital file",
    extensions: &["nd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
