use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856182: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_182,
        source_type: SourceType::Wikidata,
        name: "VariCAD Drawing (v8)",
        extensions: &["dwb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x87, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
