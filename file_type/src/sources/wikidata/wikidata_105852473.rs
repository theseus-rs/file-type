use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852473: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_473,
        source_type: SourceType::Wikidata,
        name: "Somera Graphic Format",
        extensions: &["sgf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x6F, 0x4D, 0x45, 0x52, 0x41, 0x20, 0x47, 0x52, 0x61, 0x50, 0x48,
                        0x49, 0x63, 0x20, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
