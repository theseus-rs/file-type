use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109265629: FileFormat = FileFormat {
    id: 109_265_629,
    source_type: SourceType::Wikidata,
    name: "MultiMate Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
