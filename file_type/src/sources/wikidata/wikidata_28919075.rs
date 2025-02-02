use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919075: FileFormat = FileFormat {
    id: 28_919_075,
    source_type: SourceType::Wikidata,
    name: "After Effects project template",
    extensions: &["aet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
