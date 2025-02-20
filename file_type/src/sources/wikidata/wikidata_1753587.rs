use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1753587: FileType = FileType {
    file_format: &FileFormat {
        id: 1_753_587,
        source_type: SourceType::Wikidata,
        name: "WBFS",
        extensions: &["wbfs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x42, 0x46, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
