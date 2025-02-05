use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857790: FileFormat = FileFormat {
    id: 105_857_790,
    source_type: SourceType::Wikidata,
    name: "WinAPE configuration",
    extensions: &["ini"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x52, 0x4F, 0x4D, 0x53, 0x5D, 0x0D, 0x0A, 0x43, 0x61, 0x72, 0x74, 0x72,
                    0x69, 0x64, 0x67, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
