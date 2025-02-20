use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853854: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_854,
        source_type: SourceType::Wikidata,
        name: "QuickFileCollection archive",
        extensions: &["qfc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x51, 0x46, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
