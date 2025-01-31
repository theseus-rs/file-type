use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858126: FileFormat = FileFormat {
    id: 105_858_126,
    puid: "wikidata/105858126",
    name: "Diablo 1 Item safe file format",
    extensions: &["itm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x54, 0x4D, 0x30, 0x31, 0x2E, 0x49, 0x27, 0x6C, 0x6C, 0x20, 0x67, 0x65,
                    0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6C, 0x27, 0x54, 0x68, 0x6F,
                    0x72, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
