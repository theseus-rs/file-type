use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104828509: FileType = FileType {
    file_format: &FileFormat {
        id: 104_828_509,
        source_type: SourceType::Wikidata,
        name: "PCG",
        extensions: &["pcg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x47, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
