use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859935: FileFormat = FileFormat {
    id: 105_859_935,
    puid: "wikidata/105859935",
    name: "Visual Studio analyzed Performance report",
    extensions: &["vsps"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x53, 0x50, 0x53, 0x01, 0x00, 0x00, 0x00, 0x48, 0x45, 0x41, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
