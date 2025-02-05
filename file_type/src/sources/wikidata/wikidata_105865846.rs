use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865846: FileFormat = FileFormat {
    id: 105_865_846,
    source_type: SourceType::Wikidata,
    name: "PowerVR PVR texture format (v2.x)",
    extensions: &["pvr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x52, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
