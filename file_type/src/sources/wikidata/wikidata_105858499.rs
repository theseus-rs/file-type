use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858499: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_499,
        source_type: SourceType::Wikidata,
        name: "Silicon Graphics B/W bitmap",
        extensions: &["bw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0xDA, 0x01, 0x01, 0x00, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
