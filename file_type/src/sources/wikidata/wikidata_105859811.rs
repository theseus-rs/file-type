use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859811: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_811,
        source_type: SourceType::Wikidata,
        name: "Variations actions",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x56, 0x61, 0x72, 0x69, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x73,
                        0x20, 0x61, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
