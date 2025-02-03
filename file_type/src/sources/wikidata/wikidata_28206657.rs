use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206657: FileFormat = FileFormat {
    id: 28_206_657,
    source_type: SourceType::Wikidata,
    name: "Nero CoverDesigner Document",
    extensions: &["ncd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
