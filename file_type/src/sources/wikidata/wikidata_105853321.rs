use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853321: FileFormat = FileFormat {
    id: 105_853_321,
    source_type: SourceType::Wikidata,
    name: "SNNS pattern definition",
    extensions: &["pat"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4E, 0x4E, 0x53, 0x20, 0x70, 0x61, 0x74, 0x74, 0x65, 0x72, 0x6E, 0x20,
                    0x64, 0x65, 0x66, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69,
                    0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
