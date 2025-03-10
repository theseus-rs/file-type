use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853821: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_821,
        source_type: SourceType::Wikidata,
        name: "Arahne fabric",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x46, 0x61, 0x62, 0x72, 0x69, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
