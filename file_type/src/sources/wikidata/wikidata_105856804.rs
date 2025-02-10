use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856804: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_804,
        source_type: SourceType::Wikidata,
        name: "Genbox Family History chart options",
        extensions: &["gco"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x42, 0x4F, 0x58, 0x43, 0x4F, 0x50, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
