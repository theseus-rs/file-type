use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852928: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_928,
        source_type: SourceType::Wikidata,
        name: "Splint compressed data",
        extensions: &["spl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x93, 0xB9, 0x06])],
                },
            }],
        }],
        related_formats: &[],
    },
};
