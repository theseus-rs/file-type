use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109265635: FileFormat = FileFormat {
    id: 109_265_635,
    source_type: SourceType::Wikidata,
    name: "Pro Write Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
