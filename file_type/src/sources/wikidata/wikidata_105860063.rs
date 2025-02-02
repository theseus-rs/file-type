use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860063: FileFormat = FileFormat {
    id: 105_860_063,
    source_type: SourceType::Wikidata,
    name: "ParaView VTK Image data",
    extensions: &["vti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
