use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857141: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_141,
        source_type: SourceType::Wikidata,
        name: "Hi-MD Minidisc PCM audio data container",
        extensions: &["hma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x50, 0x43, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
