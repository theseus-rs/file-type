use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850985: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_985,
        source_type: SourceType::Wikidata,
        name: "Teledisk Disk compressed image (advanced mode)",
        extensions: &["td0"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x64, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
