use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855975: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_975,
        source_type: SourceType::Wikidata,
        name: "DEB build control",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
