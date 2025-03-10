use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861737: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_737,
        source_type: SourceType::Wikidata,
        name: "J2ME RecordStore",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6D, 0x69, 0x64, 0x70, 0x2D, 0x72, 0x6D, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
