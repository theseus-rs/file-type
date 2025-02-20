use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858369: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_369,
        source_type: SourceType::Wikidata,
        name: "UN/EDIFACT",
        extensions: &["edi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
