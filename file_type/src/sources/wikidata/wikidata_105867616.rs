use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867616: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_616,
        source_type: SourceType::Wikidata,
        name: "Nintendulator movie capture",
        extensions: &["nmv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x53, 0x53, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
