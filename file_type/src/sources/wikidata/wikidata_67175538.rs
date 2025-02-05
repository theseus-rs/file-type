use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67175538: FileFormat = FileFormat {
    id: 67_175_538,
    source_type: SourceType::Wikidata,
    name: "Nero Digital file",
    extensions: &["nd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
