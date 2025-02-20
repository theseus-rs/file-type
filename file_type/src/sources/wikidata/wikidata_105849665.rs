use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849665: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_665,
        source_type: SourceType::Wikidata,
        name: "Polytron VCS logfile",
        extensions: &["cpv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x6F, 0x6C, 0x79, 0x74, 0x72, 0x6F, 0x6E, 0x20, 0x56, 0x43, 0x53,
                        0x20, 0x6C, 0x6F, 0x67, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
