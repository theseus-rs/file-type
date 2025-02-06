use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855714: FileFormat = FileFormat {
    id: 105_855_714,
    source_type: SourceType::Wikidata,
    name: "Neuratron PhotoScore Document",
    extensions: &["opt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x6E, 0x73, 0x63, 0x72, 0x70, 0x74, 0x00, 0x4E, 0x65, 0x75, 0x72,
                    0x61, 0x74, 0x72, 0x6F, 0x6E, 0x20, 0x50, 0x68, 0x6F, 0x74, 0x6F, 0x53, 0x63,
                    0x6F, 0x72, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
