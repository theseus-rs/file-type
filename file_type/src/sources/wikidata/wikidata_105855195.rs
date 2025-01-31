use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855195: FileFormat = FileFormat {
    id: 105_855_195,
    puid: "wikidata/105855195",
    name: "Flash Project",
    extensions: &["flp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x66, 0x6C, 0x61, 0x73, 0x68, 0x5F, 0x70, 0x72, 0x6F,
                    0x6A, 0x65, 0x63, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
