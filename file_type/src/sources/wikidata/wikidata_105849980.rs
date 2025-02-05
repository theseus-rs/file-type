use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849980: FileFormat = FileFormat {
    id: 105_849_980,
    source_type: SourceType::Wikidata,
    name: "HotHelp Catalog",
    extensions: &["cat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x56, 0x45, 0x52, 0x3A, 0x20, 0x48, 0x6F, 0x74, 0x48, 0x65, 0x6C, 0x70,
                    0x43, 0x61, 0x74, 0x61, 0x6C, 0x6F, 0x67, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
