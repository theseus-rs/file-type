use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867396: FileFormat = FileFormat {
    id: 105_867_396,
    puid: "wikidata/105867396",
    name: "MikroTik RouterOS Upgrade Package",
    extensions: &["npk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1E, 0xF1, 0xD0, 0xBA])],
            },
        }],
    }],
    related_formats: &[],
};
