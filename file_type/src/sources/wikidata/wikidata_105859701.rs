use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859701: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_701,
        source_type: SourceType::Wikidata,
        name: "Vis5D dataset object",
        extensions: &["v5d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x35, 0x44, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
