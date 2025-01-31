use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129571001: FileFormat = FileFormat {
    id: 129_571_001,
    puid: "wikidata/129571001",
    name: "High Level Shader Language file format",
    extensions: &["hlsl"],
    media_types: &["text/x-hlsl"],
    internal_signatures: &[],
    related_formats: &[],
};
