use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858497: FileFormat = FileFormat {
    id: 105_858_497,
    puid: "wikidata/105858497",
    name: "SuperView Graphics bitmap",
    extensions: &["svg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x56, 0x47, 0x20, 0x47, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x73, 0x20,
                    0x46, 0x69, 0x6C, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
