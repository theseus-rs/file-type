use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858970: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_970,
        source_type: SourceType::Wikidata,
        name: "Corel Photo Paint bitmap (v9)",
        extensions: &["cpt"],
        media_types: &["image/x-corel-cpt"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x54, 0x39, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
