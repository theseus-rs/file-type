use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856369: FileFormat = FileFormat {
    id: 105_856_369,
    puid: "wikidata/105856369",
    name: "YAZE disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x50, 0x4D, 0x5F, 0x44, 0x69, 0x73, 0x6B, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
