use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850515: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_515,
        source_type: SourceType::Wikidata,
        name: "PSFTools CodePage map",
        extensions: &["cp2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x53, 0x46, 0x54, 0x4F, 0x4F, 0x4C, 0x53, 0x20, 0x43, 0x4F, 0x44,
                        0x45, 0x50, 0x41, 0x47, 0x45, 0x20, 0x4D, 0x41, 0x50, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
