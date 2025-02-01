use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854535: FileFormat = FileFormat {
    id: 105_854_535,
    puid: "wikidata/105854535",
    name: "WinHki compressed archive",
    extensions: &["hki"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x5C, 0x04, 0x05, 0x14, 0x41, 0x00, 0x00, 0xFD, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
