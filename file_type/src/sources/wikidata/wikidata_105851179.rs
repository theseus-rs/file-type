use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851179: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_179,
        source_type: SourceType::Wikidata,
        name: "Boost text_oarchive serialization",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x32, 0x32, 0x20, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6C, 0x69, 0x7A, 0x61,
                        0x74, 0x69, 0x6F, 0x6E, 0x3A, 0x3A, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76,
                        0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
