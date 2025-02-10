use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851992: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_992,
        source_type: SourceType::Wikidata,
        name: "Wataroo Save state",
        extensions: &["sav"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x61, 0x74, 0x61, 0x72, 0x6F, 0x6F, 0x20, 0x3A, 0x20, 0x53, 0x61,
                        0x76, 0x65, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
