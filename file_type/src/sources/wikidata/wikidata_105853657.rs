use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853657: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_657,
        source_type: SourceType::Wikidata,
        name: "AIMP Skin (v2)",
        extensions: &["acs2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x43, 0x53, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
