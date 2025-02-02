use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600390: FileFormat = FileFormat {
    id: 28_600_390,
    source_type: SourceType::Wikidata,
    name: "Apple framework",
    extensions: &["framework"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
