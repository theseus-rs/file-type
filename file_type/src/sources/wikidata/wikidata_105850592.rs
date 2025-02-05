use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850592: FileFormat = FileFormat {
    id: 105_850_592,
    source_type: SourceType::Wikidata,
    name: "Mactive AdBase data",
    extensions: &["cdb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x42, 0x61, 0x73, 0x65, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
