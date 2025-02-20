use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853292: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_292,
        source_type: SourceType::Wikidata,
        name: "Shift Help info",
        extensions: &["sh"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x48, 0x49, 0x46, 0x54, 0x5F, 0x48, 0x45, 0x4C, 0x50, 0x0A, 0x40,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
