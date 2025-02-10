use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207028: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_028,
        source_type: SourceType::Wikidata,
        name: "Pixia",
        extensions: &["pxa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x69, 0x78, 0x69, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
