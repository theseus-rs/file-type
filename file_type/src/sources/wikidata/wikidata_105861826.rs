use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861826: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_826,
        source_type: SourceType::Wikidata,
        name: "World Construction Set model",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x43, 0x53, 0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
