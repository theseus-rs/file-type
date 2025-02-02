use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118640906: FileFormat = FileFormat {
    id: 118_640_906,
    source_type: SourceType::Wikidata,
    name: "Manga Studio 3D Object",
    extensions: &["cso"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
