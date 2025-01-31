use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859736: FileFormat = FileFormat {
    id: 105_859_736,
    puid: "wikidata/105859736",
    name: "Vocaloid Motion Data",
    extensions: &["vmd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x6F, 0x63, 0x61, 0x6C, 0x6F, 0x69, 0x64, 0x20, 0x4D, 0x6F, 0x74, 0x69,
                    0x6F, 0x6E, 0x20, 0x44, 0x61, 0x74, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
