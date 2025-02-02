use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21834748: FileFormat = FileFormat {
    id: 21_834_748,
    source_type: SourceType::Wikidata,
    name: "Adobe Color Swatch",
    extensions: &["aco"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
