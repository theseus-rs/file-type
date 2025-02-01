use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850775: FileFormat = FileFormat {
    id: 105_850_775,
    puid: "wikidata/105850775",
    name: "Microsoft Keyboard Layout Creator source (UTF-16-BE)",
    extensions: &["klc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFE, 0xFF, 0x00, 0x4B, 0x00, 0x42, 0x00, 0x44, 0x00, 0x09, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
