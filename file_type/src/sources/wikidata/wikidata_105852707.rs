use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852707: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_707,
        source_type: SourceType::Wikidata,
        name: "Sprite Pad Data (v2.0)",
        extensions: &["spd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x44, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
