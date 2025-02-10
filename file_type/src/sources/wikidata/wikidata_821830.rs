use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_821830: FileType = FileType {
    file_format: &FileFormat {
        id: 821_830,
        source_type: SourceType::Wikidata,
        name: "Symbolic Link",
        extensions: &["slk", "sylk"],
        media_types: &["application/x-sylk"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x3B, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
