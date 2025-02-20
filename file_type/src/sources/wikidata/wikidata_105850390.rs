use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850390: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_390,
        source_type: SourceType::Wikidata,
        name: "Caffeine Scheme",
        extensions: &["cscheme"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x61, 0x66, 0x66, 0x65, 0x69, 0x6E, 0x65, 0x2E, 0x53, 0x63, 0x68,
                        0x65, 0x6D, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
