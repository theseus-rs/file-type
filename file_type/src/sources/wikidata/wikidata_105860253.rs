use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860253: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_253,
        source_type: SourceType::Wikidata,
        name: "NASCAR Heat game data archive",
        extensions: &["res", "trk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x54, 0x53, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
