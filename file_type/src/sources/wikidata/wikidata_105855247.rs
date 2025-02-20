use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855247: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_247,
        source_type: SourceType::Wikidata,
        name: "AngelCode Bitmap Font (binary)",
        extensions: &["fnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
