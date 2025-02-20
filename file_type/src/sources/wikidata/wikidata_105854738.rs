use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854738: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_738,
        source_type: SourceType::Wikidata,
        name: "Amiga Money reports (v1)",
        extensions: &["amm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x4D, 0x31, 0x52, 0x45, 0x50, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
