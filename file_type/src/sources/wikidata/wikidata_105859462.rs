use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859462: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_462,
        source_type: SourceType::Wikidata,
        name: "Quartus Software Build Settings File",
        extensions: &["fsf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4F, 0x46, 0x54, 0x57, 0x41, 0x52, 0x45, 0x5F, 0x53, 0x45, 0x54,
                        0x54, 0x49, 0x4E, 0x47, 0x53, 0x0D, 0x0A, 0x7B, 0x0D, 0x0A, 0x09,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
