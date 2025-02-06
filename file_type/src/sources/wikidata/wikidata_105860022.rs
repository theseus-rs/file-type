use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860022: FileFormat = FileFormat {
    id: 105_860_022,
    source_type: SourceType::Wikidata,
    name: "Virtual CD image",
    extensions: &["000"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1F, 0xAB, 0x99, 0x00, 0x00, 0x09, 0x3D, 0x00, 0x01, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
