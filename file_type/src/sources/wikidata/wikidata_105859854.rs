use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859854: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_854,
        source_type: SourceType::Wikidata,
        name: "SymbOS VID video",
        extensions: &["vid"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x79, 0x6D, 0x56, 0x69, 0x64, 0x31, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
