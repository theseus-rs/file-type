use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862270: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_270,
        source_type: SourceType::Wikidata,
        name: "MediaPoint page",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x41, 0x47, 0x45, 0x54, 0x41, 0x4C, 0x4B, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
