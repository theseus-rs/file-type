use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207070: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_070,
        source_type: SourceType::Wikidata,
        name: "Polychrome Recursive Format",
        extensions: &["prf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x52, 0x46, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
