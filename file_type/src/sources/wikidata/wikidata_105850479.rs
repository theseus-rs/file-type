use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850479: FileFormat = FileFormat {
    id: 105_850_479,
    puid: "wikidata/105850479",
    name: "SNATCH-IT disk image",
    extensions: &["cp2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4F, 0x46, 0x54, 0x57, 0x41, 0x52, 0x45, 0x20, 0x50, 0x49, 0x52, 0x41,
                    0x54, 0x45, 0x53, 0x52, 0x65, 0x6C, 0x65, 0x61, 0x73, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
