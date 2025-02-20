use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856919: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_919,
        source_type: SourceType::Wikidata,
        name: "Google Desktop Gadget manifest",
        extensions: &["gmanifest"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
