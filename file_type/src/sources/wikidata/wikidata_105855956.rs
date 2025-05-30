use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855956: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_956,
        source_type: SourceType::Wikidata,
        name: "Ozone GUI registry Data",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x68, 0x52, 0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
