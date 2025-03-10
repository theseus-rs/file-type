use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859125: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_125,
        source_type: SourceType::Wikidata,
        name: "Audio Disk Jockey bank",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x41, 0x4E, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
