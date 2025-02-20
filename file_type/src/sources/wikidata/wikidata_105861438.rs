use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861438: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_438,
        source_type: SourceType::Wikidata,
        name: "Actor Library Definition Language",
        extensions: &["ldl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
