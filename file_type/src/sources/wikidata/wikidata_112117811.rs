use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112117811: FileType = FileType {
    file_format: &FileFormat {
        id: 112_117_811,
        source_type: SourceType::Wikidata,
        name: "ZOOM Project File",
        extensions: &["hprj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x5A, 0x4F, 0x4F, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[
                            0x70, 0x72, 0x6A, 0x65, 0x63, 0x74, 0x66, 0x69, 0x6C, 0x65,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
