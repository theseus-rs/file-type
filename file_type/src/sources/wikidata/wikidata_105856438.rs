use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856438: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_438,
        source_type: SourceType::Wikidata,
        name: "WiNGs OS application",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x08, 0x4A, 0x6F, 0x73, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
