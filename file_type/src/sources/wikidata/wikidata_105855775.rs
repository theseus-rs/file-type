use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855775: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_775,
        source_type: SourceType::Wikidata,
        name: "DNL eBook / eCatalog / eCard / eBrochure",
        extensions: &["dnl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA5, 0x9D, 0x7A, 0x18])],
                },
            }],
        }],
        related_formats: &[],
    },
};
