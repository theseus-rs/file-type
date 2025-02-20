use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861705: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_705,
        source_type: SourceType::Wikidata,
        name: "Femap Model",
        extensions: &["modfem"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x9A, 0x99, 0x99, 0x99, 0x99, 0x99, 0x24, 0x40, 0x5F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
