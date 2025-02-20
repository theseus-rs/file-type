use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864697: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_697,
        source_type: SourceType::Wikidata,
        name: "Digital Illusions game data package",
        extensions: &["pdt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x44, 0x49, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
