use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851085: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_085,
        source_type: SourceType::Wikidata,
        name: "Terragen project",
        extensions: &["tgd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x74, 0x65, 0x72, 0x72, 0x61, 0x67, 0x65, 0x6E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
