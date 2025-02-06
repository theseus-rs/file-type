use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858485: FileFormat = FileFormat {
    id: 105_858_485,
    source_type: SourceType::Wikidata,
    name: "GFA-BASIC Atari v1.00-2.02 tokenized source (protected)",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x01, 0x47, 0x66, 0x41, 0x42, 0x41, 0x53, 0x49, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
