use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1072083: FileType = FileType {
    file_format: &FileFormat {
        id: 1_072_083,
        source_type: SourceType::Wikidata,
        name: "Windows Metafile",
        extensions: &["wmf", "wmz"],
        media_types: &["image/wmf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD7, 0xCD, 0xC6, 0x9A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
