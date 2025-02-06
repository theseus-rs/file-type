use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126194224: FileFormat = FileFormat {
    id: 126_194_224,
    source_type: SourceType::Wikidata,
    name: "Safetensors",
    extensions: &["safetensors"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
