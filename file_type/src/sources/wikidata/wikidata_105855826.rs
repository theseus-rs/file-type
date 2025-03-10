use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855826: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_826,
        source_type: SourceType::Wikidata,
        name: "World Construction Set database",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x43, 0x53, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
