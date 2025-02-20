use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862500: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_500,
        source_type: SourceType::Wikidata,
        name: "Music Publisher Score",
        extensions: &["mup"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x23])],
                },
            }],
        }],
        related_formats: &[],
    },
};
