use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129571001: FileFormat = FileFormat {
    id: 129_571_001,
    source_type: SourceType::Wikidata,
    name: "High Level Shader Language file format",
    extensions: &["hlsl"],
    media_types: &["text/x-hlsl"],
    signatures: &[],
    related_formats: &[],
};
