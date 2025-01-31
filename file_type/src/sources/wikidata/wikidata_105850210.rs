use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850210: FileFormat = FileFormat {
    id: 105_850_210,
    puid: "wikidata/105850210",
    name: "WinOnCD Project",
    extensions: &["cpj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x57, 0x69, 0x6E, 0x4F, 0x6E, 0x43, 0x44, 0x20, 0x50, 0x72, 0x6F, 0x6A,
                    0x65, 0x63, 0x74, 0x5D, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
