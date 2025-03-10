use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762918: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_918,
        source_type: SourceType::Wikidata,
        name: "XBIN image/palette/font data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x42, 0x49, 0x4E, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
