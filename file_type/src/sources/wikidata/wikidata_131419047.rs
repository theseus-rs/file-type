use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131419047: FileFormat = FileFormat {
    id: 131_419_047,
    source_type: SourceType::Wikidata,
    name: "WebGPU Shading Language file format",
    extensions: &["wgsl"],
    media_types: &["text/wgsl"],
    signatures: &[],
    related_formats: &[],
};
