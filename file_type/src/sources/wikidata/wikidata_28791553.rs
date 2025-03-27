use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28791553: FileType = FileType {
    file_format: &FileFormat {
        id: 28_791_553,
        source_type: SourceType::Wikidata,
        name: "Parchive, version 2",
        extensions: &["par2"],
        media_types: &["application/x-par2"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x41, 0x52, 0x32, 0x00, 0x50, 0x4B, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
