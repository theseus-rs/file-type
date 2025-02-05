use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857605: FileFormat = FileFormat {
    id: 105_857_605,
    source_type: SourceType::Wikidata,
    name: "System 45 HP Image format tape/disk image",
    extensions: &["hpi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x05, 0x00, 0x00, 0x1E, 0x00, 0x96, 0x00, 0x01, 0x00, 0x01, 0x00, 0x17, 0x00,
                    0x1D, 0x00, 0x02, 0x00, 0x94, 0x00, 0x07, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
