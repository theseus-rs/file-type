use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857471: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_471,
        source_type: SourceType::Wikidata,
        name: "010 Editor Template List",
        extensions: &["1tl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x31, 0x54, 0x4C, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
