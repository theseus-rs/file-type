use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850600: FileFormat = FileFormat {
    id: 105_850_600,
    puid: "wikidata/105850600",
    name: "Z.A.R. Navigator Configuration",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x61, 0x72, 0x20, 0x4E, 0x61, 0x76, 0x69, 0x67, 0x61, 0x74, 0x6F, 0x72,
                    0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F,
                    0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
