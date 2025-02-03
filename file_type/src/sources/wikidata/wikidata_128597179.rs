use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128597179: FileFormat = FileFormat {
    id: 128_597_179,
    source_type: SourceType::Wikidata,
    name: "AMDGPU file",
    extensions: &["isa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
