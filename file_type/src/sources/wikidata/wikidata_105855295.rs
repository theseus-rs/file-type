use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855295: FileFormat = FileFormat {
    id: 105_855_295,
    puid: "wikidata/105855295",
    name: "ADLForms document",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x44, 0x4C, 0x46, 0x6F, 0x72, 0x6D, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
