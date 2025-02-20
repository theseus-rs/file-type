use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854555: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_555,
        source_type: SourceType::Wikidata,
        name: "Adventure Game eXecutable",
        extensions: &["agx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0xC7, 0xC1, 0x51, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
