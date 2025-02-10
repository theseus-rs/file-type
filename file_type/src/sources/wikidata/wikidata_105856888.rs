use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856888: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_888,
        source_type: SourceType::Wikidata,
        name: "Lotus 1-2-3 Graph",
        extensions: &["gph"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0F, 0x62, 0x6C, 0x64, 0x6E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
