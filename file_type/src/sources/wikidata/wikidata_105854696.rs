use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854696: FileFormat = FileFormat {
    id: 105_854_696,
    source_type: SourceType::Wikidata,
    name: "Acclaim Skeleton File",
    extensions: &["asf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
