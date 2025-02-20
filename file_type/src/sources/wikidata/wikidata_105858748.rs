use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858748: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_748,
        source_type: SourceType::Wikidata,
        name: "Adobe Bridge Thumbnail cache",
        extensions: &["bct"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6C, 0x6E, 0x62, 0x74, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
                        0x66, 0x66, 0x6D, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
