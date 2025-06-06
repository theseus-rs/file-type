use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866824: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_824,
        source_type: SourceType::Wikidata,
        name: "Atheros Profile",
        extensions: &["prf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x74, 0x68, 0x65, 0x72, 0x6F, 0x73, 0x20, 0x50, 0x72, 0x6F, 0x66,
                        0x69, 0x6C, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
