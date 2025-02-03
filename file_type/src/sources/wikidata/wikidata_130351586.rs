use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130351586: FileFormat = FileFormat {
    id: 130_351_586,
    source_type: SourceType::Wikidata,
    name: "Monkey source code file",
    extensions: &["monkey"],
    media_types: &["text/x-monkey"],
    internal_signatures: &[],
    related_formats: &[],
};
