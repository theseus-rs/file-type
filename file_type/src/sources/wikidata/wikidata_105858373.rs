use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858373: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_373,
        source_type: SourceType::Wikidata,
        name: "Encarta Encyclopedia Yearbook and Web Links update",
        extensions: &["eyb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x59, 0x42, 0x24])],
                },
            }],
        }],
        related_formats: &[],
    },
};
