use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48551303: FileFormat = FileFormat {
    id: 48_551_303,
    source_type: SourceType::Wikidata,
    name: "Word Perfect for Windows Document file format",
    extensions: &["w52", "wp", "wp5", "wpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
