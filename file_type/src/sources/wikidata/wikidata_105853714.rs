use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853714: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_714,
        source_type: SourceType::Wikidata,
        name: "Audition music",
        extensions: &["abm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x64, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x4D, 0x75, 0x73, 0x69,
                        0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
