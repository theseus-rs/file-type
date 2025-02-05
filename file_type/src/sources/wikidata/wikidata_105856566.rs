use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856566: FileFormat = FileFormat {
    id: 105_856_566,
    source_type: SourceType::Wikidata,
    name: "World Construction Set Wave",
    extensions: &["wve"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x43, 0x53, 0x57, 0x61, 0x76, 0x65, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
