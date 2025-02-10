use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853013: FileFormat = FileFormat {
    id: 105_853_013,
    source_type: SourceType::Wikidata,
    name: "Storybook Weaver Deluxe for Windows Story (v2.00)",
    extensions: &["sts", "swd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x42, 0x57, 0x20, 0x44, 0x65, 0x6C, 0x75, 0x78, 0x65, 0x20, 0x28, 0x57,
                    0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x29, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69,
                    0x6F, 0x6E, 0x20, 0x32, 0x2E, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
