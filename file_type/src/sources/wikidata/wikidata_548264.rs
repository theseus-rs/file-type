use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_548264: FileType = FileType {
    file_format: &FileFormat {
        id: 548_264,
        source_type: SourceType::Wikidata,
        name: "XML Metadata Interchange",
        extensions: &["xmi"],
        media_types: &["application/vnd.xmi+xml"],
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
