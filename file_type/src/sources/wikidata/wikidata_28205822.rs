use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205822: FileFormat = FileFormat {
    id: 28_205_822,
    source_type: SourceType::Wikidata,
    name: "CD5",
    extensions: &["cd5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
