use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853380: FileFormat = FileFormat {
    id: 105_853_380,
    puid: "wikidata/105853380",
    name: "Visual SourceSafe control file (var 1)",
    extensions: &["scc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x43, 0x43, 0x20, 0x3D, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
                    0x20, 0x61, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
