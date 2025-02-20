use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858784: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_784,
        source_type: SourceType::Wikidata,
        name: "Blue Scan drawing",
        extensions: &["blsc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4C, 0x53, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
