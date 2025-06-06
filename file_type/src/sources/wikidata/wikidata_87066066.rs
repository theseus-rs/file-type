use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87066066: FileType = FileType {
    file_format: &FileFormat {
        id: 87_066_066,
        source_type: SourceType::Wikidata,
        name: "LEADToolsCompressed Image",
        extensions: &["cmp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x45, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
