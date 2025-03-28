use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856092: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_092,
        source_type: SourceType::Wikidata,
        name: "Hyperspin DB format",
        extensions: &["dat"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x6D, 0x65, 0x6E, 0x75, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
