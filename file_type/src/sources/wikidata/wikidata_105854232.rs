use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854232: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_232,
        source_type: SourceType::Wikidata,
        name: "BMA Archiver compressed archive",
        extensions: &["bma"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x42, 0x4D, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
