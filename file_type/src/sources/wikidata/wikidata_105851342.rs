use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851342: FileFormat = FileFormat {
    id: 105_851_342,
    puid: "wikidata/105851342",
    name: "Duxbury conversion Table",
    extensions: &["tbl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x4E, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x54, 0x41,
                    0x42, 0x4C, 0x45, 0x2E, 0x0D, 0x0A, 0x43, 0x2F, 0x4D, 0x20, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
