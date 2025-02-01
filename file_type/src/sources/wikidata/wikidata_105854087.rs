use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854087: FileFormat = FileFormat {
    id: 105_854_087,
    puid: "wikidata/105854087",
    name: "16bit DOS COM COMT text converted (with text wrapper)",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x68, 0x61, 0x73,
                    0x20, 0x62, 0x65, 0x65, 0x6E, 0x20, 0x65, 0x6E, 0x63, 0x6F, 0x64, 0x65, 0x64,
                    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43, 0x4F, 0x4D,
                    0x54, 0x20, 0x65, 0x6E, 0x63, 0x6F, 0x64, 0x65, 0x72, 0x2E, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
