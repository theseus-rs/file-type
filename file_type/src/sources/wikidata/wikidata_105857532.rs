use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857532: FileFormat = FileFormat {
    id: 105_857_532,
    source_type: SourceType::Wikidata,
    name: "Indigo Renderer Material",
    extensions: &["igm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
