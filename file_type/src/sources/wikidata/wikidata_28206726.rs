use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206726: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_726,
        source_type: SourceType::Wikidata,
        name: "Portable Arbitrary Map",
        extensions: &["pam"],
        media_types: &["image/x-portable-arbitrarymap"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x37])],
                },
            }],
        }],
        related_formats: &[],
    },
};
