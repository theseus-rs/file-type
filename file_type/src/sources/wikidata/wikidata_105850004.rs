use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850004: FileFormat = FileFormat {
    id: 105_850_004,
    source_type: SourceType::Wikidata,
    name: "Microsoft NetMon v2.x traffic capture",
    extensions: &["cap"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4D, 0x42, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
