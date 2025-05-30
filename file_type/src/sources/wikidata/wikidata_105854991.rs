use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854991: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_991,
        source_type: SourceType::Wikidata,
        name: "ELI 5750 compressed archive",
        extensions: &["eli"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x72, 0x61, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
