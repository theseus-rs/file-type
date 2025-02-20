use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855853: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_853,
        source_type: SourceType::Wikidata,
        name: "SMS ASCII Dataset",
        extensions: &["dat"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x41, 0x54, 0x41, 0x53, 0x45, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
