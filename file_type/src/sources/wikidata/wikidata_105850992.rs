use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850992: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_992,
        source_type: SourceType::Wikidata,
        name: "CodeWarrior Target Data (Little Endian)",
        extensions: &["tdt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6C, 0x6F, 0x6F, 0x63])],
                },
            }],
        }],
        related_formats: &[],
    },
};
