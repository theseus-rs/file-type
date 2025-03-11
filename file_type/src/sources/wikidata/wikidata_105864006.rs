use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864006: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_006,
        source_type: SourceType::Wikidata,
        name: "MM Encoded data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x48, 0x45, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
