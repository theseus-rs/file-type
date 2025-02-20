use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855846: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_846,
        source_type: SourceType::Wikidata,
        name: "DocBook document (generic)",
        extensions: &["dbk", "xml"],
        media_types: &["application/docbook+xml"],
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
