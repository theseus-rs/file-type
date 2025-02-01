use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857636: FileFormat = FileFormat {
    id: 105_857_636,
    puid: "wikidata/105857636",
    name: "GraphPad InStat data (Windows)",
    extensions: &["isd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x72, 0x61, 0x70, 0x68, 0x50, 0x61, 0x64, 0x20, 0x49, 0x6E, 0x73, 0x74,
                    0x61, 0x74, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77,
                    0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
