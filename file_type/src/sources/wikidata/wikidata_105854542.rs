use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854542: FileFormat = FileFormat {
    id: 105_854_542,
    source_type: SourceType::Wikidata,
    name: "Audacity Block File",
    extensions: &["auf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x64, 0x61, 0x63, 0x69, 0x74, 0x79, 0x42, 0x6C, 0x6F, 0x63, 0x6B,
                    0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
