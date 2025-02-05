use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858831: FileFormat = FileFormat {
    id: 105_858_831,
    source_type: SourceType::Wikidata,
    name: "Cisco IOS XE firmware (v2.0)",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2E, 0x49, 0x4F, 0x53, 0x58, 0x45, 0x32, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
