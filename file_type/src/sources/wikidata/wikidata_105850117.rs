use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850117: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_117,
        source_type: SourceType::Wikidata,
        name: "PopCom CP/M compressed executable",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x70, 0x63, 0x31, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
