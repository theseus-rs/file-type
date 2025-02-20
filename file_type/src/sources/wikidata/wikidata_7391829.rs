use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7391829: FileType = FileType {
    file_format: &FileFormat {
        id: 7_391_829,
        source_type: SourceType::Wikidata,
        name: "Atari SoundHeader",
        extensions: &["sndh"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4E, 0x44, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
