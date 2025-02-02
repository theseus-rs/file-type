use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857137: FileFormat = FileFormat {
    id: 105_857_137,
    source_type: SourceType::Wikidata,
    name: "UniHighlighter Highlighter definition",
    extensions: &["hgl"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x55, 0x6E, 0x69, 0x48, 0x69, 0x67, 0x68, 0x6C, 0x69, 0x67, 0x68, 0x74,
                    0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
