use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849860: FileFormat = FileFormat {
    id: 105_849_860,
    source_type: SourceType::Wikidata,
    name: "Bitz and Pixels XML (ASCII)Report Template Info for collectorz.com products",
    extensions: &["cti"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x74, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x5F, 0x69, 0x6E, 0x66,
                    0x6F, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
