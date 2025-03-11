use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850996: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_996,
        source_type: SourceType::Wikidata,
        name: "Virtual ICL1900 tape",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x3C, 0xA2, 0x49, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
