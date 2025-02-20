use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858662: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_662,
        source_type: SourceType::Wikidata,
        name: "Blue Byte Archive Format",
        extensions: &["bba"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x41, 0x46, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
