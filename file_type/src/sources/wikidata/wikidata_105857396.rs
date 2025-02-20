use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857396: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_396,
        source_type: SourceType::Wikidata,
        name: "PALASM (var.4)",
        extensions: &["jed"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x4C, 0x41, 0x53, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
