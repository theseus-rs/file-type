use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859935: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_935,
        source_type: SourceType::Wikidata,
        name: "Visual Studio analyzed Performance report",
        extensions: &["vsps"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x53, 0x50, 0x53, 0x01, 0x00, 0x00, 0x00, 0x48, 0x45, 0x41, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
