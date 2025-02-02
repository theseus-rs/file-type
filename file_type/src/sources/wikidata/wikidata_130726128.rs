use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130726128: FileFormat = FileFormat {
    id: 130_726_128,
    source_type: SourceType::Wikidata,
    name: "S source code file",
    extensions: &["S"],
    media_types: &["text/S"],
    internal_signatures: &[],
    related_formats: &[],
};
