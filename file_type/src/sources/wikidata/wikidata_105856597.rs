use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856597: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_597,
        source_type: SourceType::Wikidata,
        name: "Khoros/Cantata Workspace",
        extensions: &["wksp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
