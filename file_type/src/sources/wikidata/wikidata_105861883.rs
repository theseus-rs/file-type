use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861883: FileFormat = FileFormat {
    id: 105_861_883,
    source_type: SourceType::Wikidata,
    name: "Netware 4.x Server license",
    extensions: &["mls"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4C, 0x53, 0x00, 0x49, 0x6E, 0x74, 0x72, 0x61, 0x4E, 0x65, 0x74, 0x57,
                    0x61, 0x72, 0x65, 0x20, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
