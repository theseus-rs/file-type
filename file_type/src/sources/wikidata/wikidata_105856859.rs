use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856859: FileFormat = FileFormat {
    id: 105_856_859,
    puid: "wikidata/105856859",
    name: "GUI Design Studio Catalogue",
    extensions: &["gcat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x61, 0x74, 0x61, 0x6C, 0x6F, 0x67, 0x75, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
