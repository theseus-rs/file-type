use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5639159: FileType = FileType {
    file_format: &FileFormat {
        id: 5_639_159,
        source_type: SourceType::Wikidata,
        name: "Haiku Vector Icon Format",
        extensions: &["hvif"],
        media_types: &["image/x-hvif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0x63, 0x69, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
