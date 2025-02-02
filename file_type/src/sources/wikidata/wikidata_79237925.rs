use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_79237925: FileFormat = FileFormat {
    id: 79_237_925,
    source_type: SourceType::Wikidata,
    name: "Amapi 3D model",
    extensions: &["a3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
