use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859581: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_581,
        source_type: SourceType::Wikidata,
        name: "VimCrypt encrypted data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x69, 0x6D, 0x43, 0x72, 0x79, 0x70, 0x74, 0x7E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
