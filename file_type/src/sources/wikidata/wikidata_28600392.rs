use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600392: FileFormat = FileFormat {
    id: 28_600_392,
    source_type: SourceType::Wikidata,
    name: "Applixware Graphics",
    extensions: &["ag"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x47, 0x52, 0x41, 0x50, 0x48, 0x49,
                    0x43, 0x53, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
