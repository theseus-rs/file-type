use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849649: FileFormat = FileFormat {
    id: 105_849_649,
    source_type: SourceType::Wikidata,
    name: "Compression KIT compressed C64 disk",
    extensions: &["ckit"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x53, 0x20, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
