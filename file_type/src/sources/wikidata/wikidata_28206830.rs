use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206830: FileFormat = FileFormat {
    id: 28_206_830,
    source_type: SourceType::Wikidata,
    name: "Palette Master",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
