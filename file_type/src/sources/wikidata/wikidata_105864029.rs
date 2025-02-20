use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864029: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_029,
        source_type: SourceType::Wikidata,
        name: "Monodraw diagram",
        extensions: &["monopic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0x4D, 0x4F, 0x4E, 0x4F, 0x50, 0x49, 0x43, 0x01, 0x1F, 0x8B, 0x08,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
