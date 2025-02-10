use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850384: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_384,
        source_type: SourceType::Wikidata,
        name: "Windows Cardfile database (Unicode)",
        extensions: &["crd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4B, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
