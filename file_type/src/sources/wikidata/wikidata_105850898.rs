use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850898: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_898,
        source_type: SourceType::Wikidata,
        name: "KSS music format dump",
        extensions: &["kss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x53, 0x43, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
