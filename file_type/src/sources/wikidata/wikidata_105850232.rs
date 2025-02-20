use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850232: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_232,
        source_type: SourceType::Wikidata,
        name: "Atari Cassette tape image",
        extensions: &["cas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x55, 0x4A, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
