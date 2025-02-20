use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1484072: FileType = FileType {
    file_format: &FileFormat {
        id: 1_484_072,
        source_type: SourceType::Wikidata,
        name: "ZIX archive",
        extensions: &["zix"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x49, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
