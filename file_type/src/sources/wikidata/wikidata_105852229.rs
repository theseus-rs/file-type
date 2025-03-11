use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852229: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_229,
        source_type: SourceType::Wikidata,
        name: "Audio Disk Jockey session",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x45, 0x53, 0x53, 0x49, 0x4F, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
