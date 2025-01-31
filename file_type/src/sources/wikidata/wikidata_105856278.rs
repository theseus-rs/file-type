use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856278: FileFormat = FileFormat {
    id: 105_856_278,
    puid: "wikidata/105856278",
    name: "VFloppy disk image",
    extensions: &["d88"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x38, 0x38, 0x20, 0x56, 0x46, 0x6C, 0x6F, 0x70, 0x70, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
