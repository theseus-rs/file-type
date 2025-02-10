use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859990: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_990,
        source_type: SourceType::Wikidata,
        name: "VSampler Sound Bank",
        extensions: &["vsb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x53, 0x42, 0x30, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
