use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854145: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_145,
        source_type: SourceType::Wikidata,
        name: "LAME encoded MP3 audio (ID3 v2.x tag)",
        extensions: &["mp3"],
        media_types: &["audio/mpeg3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
