use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853707: FileFormat = FileFormat {
    id: 105_853_707,
    puid: "wikidata/105853707",
    name: "AIMP PlayList",
    extensions: &["aimppl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x23, 0x00, 0x4E, 0x00, 0x61, 0x00, 0x6D, 0x00, 0x65, 0x00, 0x3A,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
