use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857853: FileFormat = FileFormat {
    id: 105_857_853,
    puid: "wikidata/105857853",
    name: "SAm Disk image",
    extensions: &["sad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x6C, 0x65, 0x79, 0x27, 0x73, 0x20, 0x64, 0x69, 0x73, 0x6B, 0x20, 0x62,
                    0x61, 0x63, 0x6B, 0x75, 0x70,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
