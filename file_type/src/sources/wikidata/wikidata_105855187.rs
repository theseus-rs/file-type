use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855187: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_187,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts graphics",
        extensions: &["fsh"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x68, 0x70, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
