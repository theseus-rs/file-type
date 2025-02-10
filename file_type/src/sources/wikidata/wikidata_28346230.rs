use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28346230: FileType = FileType {
    file_format: &FileFormat {
        id: 28_346_230,
        source_type: SourceType::Wikidata,
        name: "IMD",
        extensions: &["imd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4D, 0x44, 0x55, 0x0D, 0x0A, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
