use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853714: FileFormat = FileFormat {
    id: 105_853_714,
    puid: "wikidata/105853714",
    name: "Audition music",
    extensions: &["abm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x64, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x4D, 0x75, 0x73, 0x69, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
