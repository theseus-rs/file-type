use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850904: FileFormat = FileFormat {
    id: 105_850_904,
    source_type: SourceType::Wikidata,
    name: "Krita Paint Op Preset",
    extensions: &["kpp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49,
                    0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0xC8, 0x00, 0x00, 0x00, 0xC8, 0x08, 0x06,
                    0x00, 0x00, 0x00, 0xAD, 0x58, 0xAE, 0x9E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
