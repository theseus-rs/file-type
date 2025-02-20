use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855235: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_235,
        source_type: SourceType::Wikidata,
        name: "FCE Ultra FC0 savestate",
        extensions: &["fc0"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x43, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
