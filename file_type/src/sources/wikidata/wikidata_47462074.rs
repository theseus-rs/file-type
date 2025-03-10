use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47462074: FileType = FileType {
    file_format: &FileFormat {
        id: 47_462_074,
        source_type: SourceType::Wikidata,
        name: "Wordstar 2000 file format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7F, 0x20, 0x57, 0x53, 0x32, 0x30, 0x30, 0x30, 0xFF, 0x31, 0x2E, 0x30,
                        0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
