use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853228: FileFormat = FileFormat {
    id: 105_853_228,
    puid: "wikidata/105853228",
    name: "The Music Studio Song (Atari ST)",
    extensions: &["sng"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xCD, 0x4D, 0x73, 0x74, 0x75, 0x64, 0x69, 0x6F, 0xCD, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
