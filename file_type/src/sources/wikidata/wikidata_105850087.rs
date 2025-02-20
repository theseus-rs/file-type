use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850087: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_087,
        source_type: SourceType::Wikidata,
        name: "Clam Antivirus ByteCode signatures",
        extensions: &["cbc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x6C, 0x61, 0x6D, 0x42, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
