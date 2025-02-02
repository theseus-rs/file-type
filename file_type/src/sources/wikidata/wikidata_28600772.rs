use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600772: FileFormat = FileFormat {
    id: 28_600_772,
    source_type: SourceType::Wikidata,
    name: "EnCase hash map",
    extensions: &["EnMap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
