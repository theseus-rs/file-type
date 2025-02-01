use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859469: FileFormat = FileFormat {
    id: 105_859_469,
    puid: "wikidata/105859469",
    name: "Office Quick Access Toolbar info",
    extensions: &["qat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6D, 0x73, 0x6F, 0x3A, 0x63, 0x75, 0x73, 0x74, 0x6F, 0x6D, 0x55, 0x49,
                    0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x6D, 0x73, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
