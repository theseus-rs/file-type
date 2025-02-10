use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_72204980: FileType = FileType {
    file_format: &FileFormat {
        id: 72_204_980,
        source_type: SourceType::Wikidata,
        name: "Lotus Forms Template",
        extensions: &["ltm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x4F, 0x54, 0x55, 0x53, 0x46, 0x4F, 0x52, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
