use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849769: FileFormat = FileFormat {
    id: 105_849_769,
    source_type: SourceType::Wikidata,
    name: "Cartoon Studio Script",
    extensions: &["css"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x61, 0x72, 0x74, 0x6F, 0x6F, 0x6E, 0x44, 0x61, 0x74, 0x61, 0x4E, 0x45,
                    0x57, 0x0A, 0x43, 0x61, 0x72, 0x74, 0x6F, 0x6F, 0x6E, 0x53, 0x74, 0x75, 0x64,
                    0x69, 0x6F, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
