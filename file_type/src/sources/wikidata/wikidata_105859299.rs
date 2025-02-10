use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859299: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_299,
        source_type: SourceType::Wikidata,
        name: "Mallard BASIC tokenized source (protected) (new)",
        extensions: &["bas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFC, 0x04, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
