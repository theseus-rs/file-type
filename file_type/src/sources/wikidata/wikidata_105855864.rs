use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855864: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_864,
        source_type: SourceType::Wikidata,
        name: "Shareaza thumb",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x41, 0x5A, 0x41, 0x54, 0x44, 0x42, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
