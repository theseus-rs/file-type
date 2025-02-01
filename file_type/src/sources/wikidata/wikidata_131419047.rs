use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131419047: FileFormat = FileFormat {
    id: 131_419_047,
    puid: "wikidata/131419047",
    name: "WebGPU Shading Language file format",
    extensions: &["wgsl"],
    media_types: &["text/wgsl"],
    internal_signatures: &[],
    related_formats: &[],
};
