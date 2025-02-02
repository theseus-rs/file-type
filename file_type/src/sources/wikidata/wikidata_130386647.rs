use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130386647: FileFormat = FileFormat {
    id: 130_386_647,
    source_type: SourceType::Wikidata,
    name: "NuSMV file format",
    extensions: &["smv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
