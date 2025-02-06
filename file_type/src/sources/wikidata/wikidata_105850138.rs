use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850138: FileFormat = FileFormat {
    id: 105_850_138,
    source_type: SourceType::Wikidata,
    name: "OPTune Configuration",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x50, 0x54, 0x55, 0x4E, 0x45, 0x20, 0x43, 0x4F, 0x4E, 0x46, 0x49, 0x47,
                    0x55, 0x52, 0x41, 0x54, 0x49, 0x4F, 0x4E, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
