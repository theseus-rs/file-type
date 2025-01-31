use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864990: FileFormat = FileFormat {
    id: 105_864_990,
    puid: "wikidata/105864990",
    name: "Palm TealMeal",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x44, 0x61, 0x74, 0x61, 0x54, 0x6C, 0x4D, 0x6C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
