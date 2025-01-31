use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864891: FileFormat = FileFormat {
    id: 105_864_891,
    puid: "wikidata/105864891",
    name: "Personal Media Suite encoded file",
    extensions: &["pms"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x09, 0x00, 0x08, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
