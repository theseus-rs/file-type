use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858902: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_902,
        source_type: SourceType::Wikidata,
        name: "Separations output format bitmap",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x65, 0x70, 0x4F, 0x75, 0x74, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
