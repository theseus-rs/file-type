use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854924: FileFormat = FileFormat {
    id: 105_854_924,
    source_type: SourceType::Wikidata,
    name: "Psion serie 3 AICA Sound Compressor audio",
    extensions: &["aik"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x49, 0x43, 0x41, 0x53, 0x6F, 0x75, 0x6E, 0x64, 0x46, 0x69, 0x6C, 0x65,
                    0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
