use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71178742: FileFormat = FileFormat {
    id: 71_178_742,
    source_type: SourceType::Wikidata,
    name: "PIPE2 file format",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
