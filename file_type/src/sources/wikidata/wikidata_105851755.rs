use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851755: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_755,
        source_type: SourceType::Wikidata,
        name: "Playmation sculpture/model",
        extensions: &["seg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x43, 0x55, 0x4C, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
