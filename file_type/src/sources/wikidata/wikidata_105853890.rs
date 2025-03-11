use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853890: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_890,
        source_type: SourceType::Wikidata,
        name: "TTComp archive compressed (bin-4K)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x06])],
                },
            }],
        }],
        related_formats: &[],
    },
};
