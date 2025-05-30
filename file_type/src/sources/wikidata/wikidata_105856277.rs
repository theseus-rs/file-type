use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856277: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_277,
        source_type: SourceType::Wikidata,
        name: "VariCAD Drawing (v7)",
        extensions: &["dwb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x87, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
