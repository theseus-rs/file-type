use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859739: FileFormat = FileFormat {
    id: 105_859_739,
    source_type: SourceType::Wikidata,
    name: "VMware configuration (alternate)",
    extensions: &["vmx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2E, 0x65, 0x6E, 0x63, 0x6F, 0x64, 0x69, 0x6E, 0x67, 0x20, 0x3D, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
