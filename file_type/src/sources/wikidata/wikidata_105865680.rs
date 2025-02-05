use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865680: FileFormat = FileFormat {
    id: 105_865_680,
    source_type: SourceType::Wikidata,
    name: "Pencil project",
    extensions: &["pcl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x50, 0x65, 0x6E,
                    0x63, 0x69, 0x6C, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
