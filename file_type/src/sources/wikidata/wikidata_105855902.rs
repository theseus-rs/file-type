use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855902: FileFormat = FileFormat {
    id: 105_855_902,
    source_type: SourceType::Wikidata,
    name: "Chessmaster 3000 Opening Book",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x68, 0x65, 0x73, 0x73, 0x6D, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x33,
                    0x30, 0x30, 0x30, 0x20, 0x4F, 0x70, 0x65, 0x6E, 0x69, 0x6E, 0x67, 0x20, 0x42,
                    0x6F, 0x6F, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
