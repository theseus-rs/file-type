use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859556: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_556,
        source_type: SourceType::Wikidata,
        name: "Nintendo DS MPEG Video",
        extensions: &["dpg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x50, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
