use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128597179: FileFormat = FileFormat {
    id: 128_597_179,
    source_type: SourceType::Wikidata,
    name: "AMDGPU file",
    extensions: &["isa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
