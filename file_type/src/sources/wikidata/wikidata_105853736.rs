use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853736: FileFormat = FileFormat {
    id: 105_853_736,
    source_type: SourceType::Wikidata,
    name: "Grace project file",
    extensions: &["agr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x47, 0x72, 0x61, 0x63, 0x65, 0x20, 0x70, 0x72, 0x6F, 0x6A, 0x65,
                    0x63, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
