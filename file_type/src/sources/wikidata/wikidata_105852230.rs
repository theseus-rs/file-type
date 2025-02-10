use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852230: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_230,
        source_type: SourceType::Wikidata,
        name: "Turbo Silver v3 Script",
        extensions: &["scr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x49, 0x4C, 0x56, 0x45, 0x52, 0x20, 0x33, 0x0A, 0x53, 0x43, 0x52,
                        0x50, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
