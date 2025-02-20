use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855820: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_820,
        source_type: SourceType::Wikidata,
        name: "VariCAD Drawing (2020)",
        extensions: &["dwb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x87, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
