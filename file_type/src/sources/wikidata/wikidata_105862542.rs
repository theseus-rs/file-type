use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862542: FileFormat = FileFormat {
    id: 105_862_542,
    puid: "wikidata/105862542",
    name: "OS/2 help Message (generic)",
    extensions: &["msg"],
    media_types: &["application/x-os2-msg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x4D, 0x4B, 0x4D, 0x53, 0x47, 0x46, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
