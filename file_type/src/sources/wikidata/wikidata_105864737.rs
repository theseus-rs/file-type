use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864737: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_737,
        source_type: SourceType::Wikidata,
        name: "PatchMeister Driver (v2)",
        extensions: &["pmdriver"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x44, 0x32, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
