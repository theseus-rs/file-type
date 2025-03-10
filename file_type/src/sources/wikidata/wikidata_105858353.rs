use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858353: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_353,
        source_type: SourceType::Wikidata,
        name: "Mac OS X Mach-O 32bit PPC executable",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xED, 0xFA, 0xCF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
