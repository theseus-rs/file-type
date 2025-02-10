use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851133: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_133,
        source_type: SourceType::Wikidata,
        name: "3D Ultra Cool data file",
        extensions: &["tbv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x42, 0x56, 0x6F, 0x6C, 0x75, 0x6D, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
