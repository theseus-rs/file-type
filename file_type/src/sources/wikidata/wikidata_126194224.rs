use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126194224: FileFormat = FileFormat {
    id: 126_194_224,
    source_type: SourceType::Wikidata,
    name: "Safetensors",
    extensions: &["safetensors"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
