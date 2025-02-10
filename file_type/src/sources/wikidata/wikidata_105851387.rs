use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851387: FileFormat = FileFormat {
    id: 105_851_387,
    source_type: SourceType::Wikidata,
    name: "Sniffer capture",
    extensions: &["snf", "trc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x52, 0x53, 0x4E, 0x49, 0x46, 0x46, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20,
                    0x20, 0x20, 0x20, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
