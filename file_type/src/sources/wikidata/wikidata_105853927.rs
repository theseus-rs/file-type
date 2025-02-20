use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853927: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_927,
        source_type: SourceType::Wikidata,
        name: "ArcView Legend",
        extensions: &["avl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x33, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
