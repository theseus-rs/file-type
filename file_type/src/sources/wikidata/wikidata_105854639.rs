use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854639: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_639,
        source_type: SourceType::Wikidata,
        name: "UltraCompressor 2 Archive",
        extensions: &["uc2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x43, 0x32, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
