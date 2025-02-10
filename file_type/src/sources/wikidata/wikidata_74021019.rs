use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_74021019: FileType = FileType {
    file_format: &FileFormat {
        id: 74_021_019,
        source_type: SourceType::Wikidata,
        name: "RealMedia meta file",
        extensions: &["ram"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x74, 0x73, 0x70, 0x3A, 0x2F, 0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
