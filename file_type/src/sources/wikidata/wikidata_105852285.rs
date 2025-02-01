use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852285: FileFormat = FileFormat {
    id: 105_852_285,
    puid: "wikidata/105852285",
    name: "The Music Studio Sound (Atari ST)",
    extensions: &["snd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xCD, 0x4D, 0x73, 0x74, 0x75, 0x64, 0x69, 0x6F, 0xCD, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
