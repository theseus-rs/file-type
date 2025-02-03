use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122408622: FileFormat = FileFormat {
    id: 122_408_622,
    source_type: SourceType::Wikidata,
    name: "68K Symbol File",
    extensions: &["sym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
