use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851145: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_145,
        source_type: SourceType::Wikidata,
        name: "SSI packed Library format",
        extensions: &["glb", "tlb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4C, 0x49, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
