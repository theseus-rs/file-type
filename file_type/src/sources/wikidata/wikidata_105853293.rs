use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853293: FileFormat = FileFormat {
    id: 105_853_293,
    puid: "wikidata/105853293",
    name: "no$gba Nintendo Gameboy Advance emulator backup",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x6F, 0x63, 0x61, 0x73, 0x68, 0x47, 0x62, 0x61, 0x42, 0x61, 0x63, 0x6B,
                    0x75, 0x70, 0x4D, 0x65, 0x64, 0x69, 0x61, 0x53, 0x61, 0x76, 0x44, 0x61, 0x74,
                    0x61, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
