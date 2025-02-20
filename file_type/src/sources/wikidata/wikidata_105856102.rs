use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856102: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_102,
        source_type: SourceType::Wikidata,
        name: "AutoCAD R1.0 Drawing",
        extensions: &["dwg"],
        media_types: &["application/x-autocad"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x43, 0x30, 0x2E, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
