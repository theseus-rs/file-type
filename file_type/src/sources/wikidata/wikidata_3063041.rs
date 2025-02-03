use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3063041: FileFormat = FileFormat {
    id: 3_063_041,
    source_type: SourceType::Wikidata,
    name: "Filmbox",
    extensions: &["fbx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
