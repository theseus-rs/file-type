use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28009498: FileFormat = FileFormat {
    id: 28_009_498,
    source_type: SourceType::Wikidata,
    name: "Zj-Stream",
    extensions: &["prn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
