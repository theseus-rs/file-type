use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851421: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_421,
        source_type: SourceType::Wikidata,
        name: "Turbo Debugger configuration",
        extensions: &["td2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x75, 0x72, 0x62, 0x6F, 0x20, 0x44, 0x65, 0x62, 0x75, 0x67, 0x67,
                        0x65, 0x72, 0x20, 0x63, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
                        0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
