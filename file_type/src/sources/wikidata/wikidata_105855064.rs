use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855064: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_064,
        source_type: SourceType::Wikidata,
        name: "BIS WSS PCM audio",
        extensions: &["wss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x53, 0x53, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
