use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866999: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_999,
        source_type: SourceType::Wikidata,
        name: "Neo Content file",
        extensions: &["nwp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x45, 0x4F, 0x20, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
