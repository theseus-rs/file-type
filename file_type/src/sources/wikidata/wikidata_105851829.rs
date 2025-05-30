use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851829: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_829,
        source_type: SourceType::Wikidata,
        name: "ShipInBottle compressed file",
        extensions: &["sib"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x68, 0x49, 0x6E, 0x42, 0x6F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
