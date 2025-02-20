use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861080: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_080,
        source_type: SourceType::Wikidata,
        name: "Windows application log",
        extensions: &["lgc", "lgd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x0D, 0x0A, 0x6F, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
