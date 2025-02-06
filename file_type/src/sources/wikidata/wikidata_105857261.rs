use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857261: FileFormat = FileFormat {
    id: 105_857_261,
    source_type: SourceType::Wikidata,
    name: "RTFGEN Help Source",
    extensions: &["hps"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4F, 0x50, 0x54, 0x49, 0x4F, 0x4E, 0x53, 0x5D, 0x0D, 0x0A, 0x20, 0x20,
                    0x43, 0x4F, 0x4D, 0x50, 0x52, 0x45, 0x53, 0x53, 0x20, 0x3D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
