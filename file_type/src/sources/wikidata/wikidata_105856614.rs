use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856614: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_614,
        source_type: SourceType::Wikidata,
        name: "Webpack bundle",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x77, 0x65, 0x62, 0x70, 0x61, 0x63, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
