use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124970024: FileFormat = FileFormat {
    id: 124_970_024,
    source_type: SourceType::Wikidata,
    name: "MIX metadata file",
    extensions: &["mixmeta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
