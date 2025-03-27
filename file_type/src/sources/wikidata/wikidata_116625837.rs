use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116625837: FileType = FileType {
    file_format: &FileFormat {
        id: 116_625_837,
        source_type: SourceType::Wikidata,
        name: "Quite OK Audio Format",
        extensions: &["qoa"],
        media_types: &[
            "http://www.wikidata.org/.well-known/genid/b158173902ebf2b59c9d83758c150d49",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x71, 0x6F, 0x61, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
