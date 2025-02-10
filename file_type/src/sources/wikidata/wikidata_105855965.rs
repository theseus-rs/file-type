use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855965: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_965,
        source_type: SourceType::Wikidata,
        name: "FL Studio Drum Patch (v2)",
        extensions: &["dmpatch"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x50, 0x4D, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
