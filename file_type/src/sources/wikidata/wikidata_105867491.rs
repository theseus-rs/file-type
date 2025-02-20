use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867491: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_491,
        source_type: SourceType::Wikidata,
        name: "Nintendo Switch Submission Package",
        extensions: &["nsp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x53, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
