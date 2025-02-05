use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850553: FileFormat = FileFormat {
    id: 105_850_553,
    source_type: SourceType::Wikidata,
    name: "Chessmaster saved game (10th ed.)",
    extensions: &["cmg"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x68, 0x65, 0x73, 0x73, 0x6D, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x31,
                    0x30, 0x74, 0x68, 0x20, 0x45, 0x64, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
