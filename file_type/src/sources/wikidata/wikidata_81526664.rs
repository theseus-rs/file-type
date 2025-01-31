use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81526664: FileFormat = FileFormat {
    id: 81_526_664,
    puid: "wikidata/81526664",
    name: "Palm Desktop DateBook",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xBE, 0xBA, 0xFE, 0xCA, 0x0F, 0x50, 0x61, 0x6C, 0x6D, 0x53, 0x47, 0x20, 0x44,
                    0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
