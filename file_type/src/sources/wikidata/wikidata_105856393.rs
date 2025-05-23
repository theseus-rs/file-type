use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856393: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_393,
        source_type: SourceType::Wikidata,
        name: "WebEx Recording",
        extensions: &["wot", "wrf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x4F, 0x54, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
