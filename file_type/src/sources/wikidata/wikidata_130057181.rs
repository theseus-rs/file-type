use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130057181: FileFormat = FileFormat {
    id: 130_057_181,
    source_type: SourceType::Wikidata,
    name: "K source code file",
    extensions: &["k"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
