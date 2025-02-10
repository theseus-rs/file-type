use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858767: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_767,
        source_type: SourceType::Wikidata,
        name: "16 bit adaptive RLE compressed bitmap",
        extensions: &["jmg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x4D, 0x47, 0x20, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
