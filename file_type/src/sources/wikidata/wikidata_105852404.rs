use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852404: FileFormat = FileFormat {
    id: 105_852_404,
    puid: "wikidata/105852404",
    name: "Skype Extra",
    extensions: &["sparc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x77, 0x77, 0x77, 0x77, 0x03, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
