use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854514: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_514,
        source_type: SourceType::Wikidata,
        name: "MP3 audio (ID3 v1.x tag)",
        extensions: &["mp3"],
        media_types: &["audio/mpeg3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
