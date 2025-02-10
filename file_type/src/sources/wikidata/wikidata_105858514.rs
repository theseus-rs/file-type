use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858514: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_514,
        source_type: SourceType::Wikidata,
        name: "Houdini Binary LUT (linear)",
        extensions: &["blut"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x4C, 0x55, 0x54, 0x00, 0x00, 0x00, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
