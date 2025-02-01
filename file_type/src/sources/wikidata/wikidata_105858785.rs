use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858785: FileFormat = FileFormat {
    id: 105_858_785,
    puid: "wikidata/105858785",
    name: "BlockHashLoc recovery info",
    extensions: &["bhl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x6C, 0x6F, 0x63, 0x6B, 0x48, 0x61, 0x73, 0x68, 0x4C, 0x6F, 0x63, 0x1A,
                    0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
