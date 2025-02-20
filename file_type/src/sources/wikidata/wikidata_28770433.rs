use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770433: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_433,
        source_type: SourceType::Wikidata,
        name: "MARCXML",
        extensions: &["mrcx"],
        media_types: &["application/marc", "application/marcxml+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
