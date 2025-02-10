use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853860: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_860,
        source_type: SourceType::Wikidata,
        name: "Hamarsoft HAP compressed archive (v3.00)",
        extensions: &["hap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x91, 0x33, 0x48, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
