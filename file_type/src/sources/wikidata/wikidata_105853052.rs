use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853052: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_052,
        source_type: SourceType::Wikidata,
        name: "TIENCR format",
        extensions: &["senc", "tiencr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x49, 0x45, 0x4E, 0x43, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
