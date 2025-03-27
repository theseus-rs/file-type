use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1440353: FileType = FileType {
    file_format: &FileFormat {
        id: 1_440_353,
        source_type: SourceType::Wikidata,
        name: "Fractal Image Format",
        extensions: &["fif"],
        media_types: &[
            "application/fractals",
            "application/octet-stream",
            "image/fif",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x49, 0x46, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
