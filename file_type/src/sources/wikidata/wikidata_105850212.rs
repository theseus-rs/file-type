use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850212: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_212,
        source_type: SourceType::Wikidata,
        name: "AmiDock Configuration",
        extensions: &["config"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4F, 0x43, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
