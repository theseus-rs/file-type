use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854120: FileFormat = FileFormat {
    id: 105_854_120,
    source_type: SourceType::Wikidata,
    name: "ANSYS model data",
    extensions: &["ans"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x20, 0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x3A,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
