use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850361: FileFormat = FileFormat {
    id: 105_850_361,
    source_type: SourceType::Wikidata,
    name: "QV Map Calibration data",
    extensions: &["cal"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x43, 0x61, 0x6C, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
