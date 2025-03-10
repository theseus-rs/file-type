use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852930: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_930,
        source_type: SourceType::Wikidata,
        name: "SuperStor compressed volume",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEB, 0x2D, 0x90, 0x41, 0x44, 0x44, 0x53, 0x54, 0x4F, 0x52, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
