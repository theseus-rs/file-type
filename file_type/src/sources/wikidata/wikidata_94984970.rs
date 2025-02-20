use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_94984970: FileType = FileType {
    file_format: &FileFormat {
        id: 94_984_970,
        source_type: SourceType::Wikidata,
        name: "IGC",
        extensions: &["igc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
