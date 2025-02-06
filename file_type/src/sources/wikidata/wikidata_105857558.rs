use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857558: FileFormat = FileFormat {
    id: 105_857_558,
    source_type: SourceType::Wikidata,
    name: "A2R disk image (v1)",
    extensions: &["a2r"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x32, 0x52, 0x31, 0xFF, 0x0A, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
