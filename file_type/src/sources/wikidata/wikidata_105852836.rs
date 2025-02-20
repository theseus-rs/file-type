use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852836: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_836,
        source_type: SourceType::Wikidata,
        name: "AmigaKonto Specification",
        extensions: &["spec"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4F,
                        0x4E, 0x5F, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
