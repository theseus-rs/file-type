use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856363: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_363,
        source_type: SourceType::Wikidata,
        name: "Furcadia DragonSpeak Script",
        extensions: &["ds"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x53, 0x50, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
