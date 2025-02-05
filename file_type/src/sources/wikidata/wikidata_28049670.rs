use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049670: FileFormat = FileFormat {
    id: 28_049_670,
    source_type: SourceType::Wikidata,
    name: "Autodesk 3D Studio ASCII format",
    extensions: &["asc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
