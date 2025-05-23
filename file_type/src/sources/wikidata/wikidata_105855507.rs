use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855507: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_507,
        source_type: SourceType::Wikidata,
        name: "Fujitsu composite Font",
        extensions: &["fon"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x55, 0x4A, 0x49, 0x54, 0x53, 0x55, 0x20, 0x43, 0x4F, 0x4D, 0x50,
                        0x4F, 0x53, 0x49, 0x54, 0x45, 0x20, 0x46, 0x4F, 0x4E, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
