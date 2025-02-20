use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860119: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_119,
        source_type: SourceType::Wikidata,
        name: "Emu80 RK snapshot",
        extensions: &["rss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x4B, 0x53, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
