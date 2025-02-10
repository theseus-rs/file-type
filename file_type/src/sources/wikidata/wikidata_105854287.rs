use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_287,
        source_type: SourceType::Wikidata,
        name: "Jetico BCArchive encrypted archive",
        extensions: &["bca"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x65, 0x74, 0x69, 0x63, 0x6F, 0x45, 0x6E, 0x63, 0x72, 0x79, 0x70,
                        0x74, 0x41, 0x72, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
