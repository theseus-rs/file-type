use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858692: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_692,
        source_type: SourceType::Wikidata,
        name: "JPEG-2000 Code Stream bitmap",
        extensions: &["j2c", "j2k", "jpc"],
        media_types: &["image/jp2", "image/x-jp2-codestream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0x4F, 0xFF, 0x51])],
                },
            }],
        }],
        related_formats: &[],
    },
};
