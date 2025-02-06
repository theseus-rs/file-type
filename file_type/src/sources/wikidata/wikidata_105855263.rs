use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855263: FileFormat = FileFormat {
    id: 105_855_263,
    source_type: SourceType::Wikidata,
    name: "Phoenix Visual Designer form",
    extensions: &["frm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x31, 0x2E, 0x30, 0x0D, 0x0A,
                    0x55, 0x4E, 0x49, 0x54, 0x53, 0x20, 0x50, 0x69, 0x78, 0x65, 0x6C, 0x73, 0x0D,
                    0x0A, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x56, 0x50, 0x42, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
