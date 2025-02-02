use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206733: FileFormat = FileFormat {
    id: 28_206_733,
    source_type: SourceType::Wikidata,
    name: "Newsroom",
    extensions: &["nsr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
