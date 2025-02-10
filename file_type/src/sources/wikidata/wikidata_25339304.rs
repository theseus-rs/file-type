use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_25339304: FileType = FileType {
    file_format: &FileFormat {
        id: 25_339_304,
        source_type: SourceType::Wikidata,
        name: "Timed Text Markup Language",
        extensions: &["dxfp", "ttml", "xml"],
        media_types: &["application/ttml+xml"],
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
