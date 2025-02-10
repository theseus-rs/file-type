use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856982: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_982,
        source_type: SourceType::Wikidata,
        name: "GridMove grid template",
        extensions: &["grid"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x47, 0x72, 0x6F, 0x75, 0x70, 0x73, 0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
