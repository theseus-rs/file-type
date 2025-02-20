use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855353: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_353,
        source_type: SourceType::Wikidata,
        name: "Fiasco Database: print report definitions",
        extensions: &["fpr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x50, 0x52, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
