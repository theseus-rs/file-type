use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857519: FileFormat = FileFormat {
    id: 105_857_519,
    source_type: SourceType::Wikidata,
    name: "Novell 16-bit LAN driver Installer data",
    extensions: &["ins"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x6E, 0x53, 0x5F, 0x53, 0x74, 0x41, 0x72, 0x54, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
