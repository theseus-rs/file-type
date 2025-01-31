use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128597179: FileFormat = FileFormat {
    id: 128_597_179,
    puid: "wikidata/128597179",
    name: "AMDGPU file",
    extensions: &["isa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
