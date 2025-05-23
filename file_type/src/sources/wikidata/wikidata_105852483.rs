use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852483: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_483,
        source_type: SourceType::Wikidata,
        name: "Datamat database structure",
        extensions: &["sts"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x49, 0x4C, 0x45, 0x00, 0x00, 0x00, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
