use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205640: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_640,
        source_type: SourceType::Wikidata,
        name: "Xcursor",
        extensions: &[],
        media_types: &["image/x-xcursor"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x63, 0x75, 0x72])],
                },
            }],
        }],
        related_formats: &[],
    },
};
