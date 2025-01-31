use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849784: FileFormat = FileFormat {
    id: 105_849_784,
    puid: "wikidata/105849784",
    name: "CWTool disk image (text)",
    extensions: &["cwt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x63, 0x77, 0x74, 0x6F, 0x6F, 0x6C, 0x20, 0x72, 0x61, 0x77, 0x20,
                    0x74, 0x65, 0x78, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
