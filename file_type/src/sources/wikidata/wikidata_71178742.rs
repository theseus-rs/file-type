use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71178742: FileFormat = FileFormat {
    id: 71_178_742,
    source_type: SourceType::Wikidata,
    name: "PIPE2 file format",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
