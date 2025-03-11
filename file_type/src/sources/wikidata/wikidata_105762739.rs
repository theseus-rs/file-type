use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762739: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_739,
        source_type: SourceType::Wikidata,
        name: "XTrkCAD demo",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
