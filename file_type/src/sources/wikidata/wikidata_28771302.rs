use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28771302: FileFormat = FileFormat {
    id: 28_771_302,
    source_type: SourceType::Wikidata,
    name: "Matlab figure",
    extensions: &["fig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
