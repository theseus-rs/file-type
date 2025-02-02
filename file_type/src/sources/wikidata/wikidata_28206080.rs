use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206080: FileFormat = FileFormat {
    id: 28_206_080,
    source_type: SourceType::Wikidata,
    name: "PI6",
    extensions: &["PI6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
