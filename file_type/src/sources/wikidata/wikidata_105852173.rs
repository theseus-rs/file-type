use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852173: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_173,
        source_type: SourceType::Wikidata,
        name: "Scidb game moves info",
        extensions: &["scg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x63, 0x69, 0x64, 0x62, 0x2E, 0x67, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
