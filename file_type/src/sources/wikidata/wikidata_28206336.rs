use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206336: FileFormat = FileFormat {
    id: 28_206_336,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Blue Component",
    extensions: &["b"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
