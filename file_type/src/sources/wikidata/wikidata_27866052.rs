use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27866052: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_052,
        source_type: SourceType::Wikidata,
        name: "bzip2 Archive",
        extensions: &["bz2"],
        media_types: &["application/x-bzip2"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x5A, 0x68])],
                },
            }],
        }],
        related_formats: &[],
    },
};
