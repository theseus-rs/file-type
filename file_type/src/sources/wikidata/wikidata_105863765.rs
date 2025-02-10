use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863765: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_765,
        source_type: SourceType::Wikidata,
        name: "Symbian Machine-readable File graphics",
        extensions: &["mif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x23, 0x23, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
