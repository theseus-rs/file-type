use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858747: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_747,
        source_type: SourceType::Wikidata,
        name: "Boom effect",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x30, 0x30, 0x4D, 0x00, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
