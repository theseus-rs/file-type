use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858132: FileFormat = FileFormat {
    id: 105_858_132,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine sound (v1.0)",
    extensions: &["wav"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x41, 0x56, 0x43, 0x56, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
