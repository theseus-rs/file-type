use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851193: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_193,
        source_type: SourceType::Wikidata,
        name: "TestGen data",
        extensions: &["tst"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x47, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
