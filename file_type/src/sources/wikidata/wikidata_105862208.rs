use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862208: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_208,
        source_type: SourceType::Wikidata,
        name: "MG!2 compressed",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x47, 0x21, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
