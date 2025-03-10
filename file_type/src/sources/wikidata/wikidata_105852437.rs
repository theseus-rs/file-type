use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852437: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_437,
        source_type: SourceType::Wikidata,
        name: "StormEd settings",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x74, 0x6F, 0x72, 0x6D, 0x45, 0x64, 0x20, 0x53, 0x65, 0x74, 0x74,
                        0x69, 0x6E, 0x67, 0x73, 0x20, 0x28,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
