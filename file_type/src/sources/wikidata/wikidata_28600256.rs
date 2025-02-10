use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28600256: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_256,
        source_type: SourceType::Wikidata,
        name: "ASCII Encoded HP 48 Object",
        extensions: &["asc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x25, 0x48, 0x50, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
