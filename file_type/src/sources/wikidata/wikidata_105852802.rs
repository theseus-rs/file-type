use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852802: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_802,
        source_type: SourceType::Wikidata,
        name: "Aegis Videoscape 3D Set",
        extensions: &["set"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x44, 0x53, 0x31, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
