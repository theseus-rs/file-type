use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049670: FileFormat = FileFormat {
    id: 28_049_670,
    source_type: SourceType::Wikidata,
    name: "Autodesk 3D Studio ASCII format",
    extensions: &["asc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
