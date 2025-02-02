use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105582538: FileFormat = FileFormat {
    id: 105_582_538,
    source_type: SourceType::Wikidata,
    name: "JavaScript modules",
    extensions: &["mjs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
