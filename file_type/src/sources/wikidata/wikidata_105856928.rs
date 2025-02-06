use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856928: FileFormat = FileFormat {
    id: 105_856_928,
    source_type: SourceType::Wikidata,
    name: "McAfee AV Pattern update (newer engine)",
    extensions: &["gem"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x65, 0x6E, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x44, 0x41, 0x54, 0x20, 0x28,
                    0x43, 0x29, 0x20, 0x4D, 0x63, 0x41, 0x66, 0x65, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
