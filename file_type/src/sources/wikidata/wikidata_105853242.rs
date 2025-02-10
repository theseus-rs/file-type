use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853242: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_242,
        source_type: SourceType::Wikidata,
        name: "The Witcher 3: Wild Hunt game save",
        extensions: &["sav"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4E, 0x46, 0x48, 0x46, 0x5A, 0x4C, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
