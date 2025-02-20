use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864790: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_790,
        source_type: SourceType::Wikidata,
        name: "Particles format (big-endian)",
        extensions: &["pb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFF, 0xFF, 0x98])],
                },
            }],
        }],
        related_formats: &[],
    },
};
