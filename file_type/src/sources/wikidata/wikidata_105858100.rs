use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858100: FileFormat = FileFormat {
    id: 105_858_100,
    puid: "wikidata/105858100",
    name: "Agat Emulator virtual Disk",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x67, 0x61, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6D, 0x75, 0x6C, 0x61, 0x74,
                    0x6F, 0x72, 0x20, 0x76, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x20, 0x64, 0x69,
                    0x73, 0x6B, 0x0D, 0x0A, 0x1A, 0x41, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
