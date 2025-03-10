use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852244: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_244,
        source_type: SourceType::Wikidata,
        name: "Spectrum File System boot sector (var. 2)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x18, 0x7E, 0x53, 0x46, 0x53, 0x2D, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
