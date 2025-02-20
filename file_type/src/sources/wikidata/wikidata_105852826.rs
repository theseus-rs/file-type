use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852826: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_826,
        source_type: SourceType::Wikidata,
        name: "First Choice SpreadSheet",
        extensions: &["ss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x4A, 0x45, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
