use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_83883611: FileType = FileType {
    file_format: &FileFormat {
        id: 83_883_611,
        source_type: SourceType::Wikidata,
        name: "Raw Flux Image",
        extensions: &["rfi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x46, 0x49, 0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
