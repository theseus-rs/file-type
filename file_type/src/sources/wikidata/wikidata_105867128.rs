use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867128: FileFormat = FileFormat {
    id: 105_867_128,
    source_type: SourceType::Wikidata,
    name: "Nord Stage 2 Program",
    extensions: &["ns2p"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x42, 0x49, 0x4E, 0x00, 0x00, 0x00, 0x00, 0x6E, 0x73, 0x32, 0x70,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
