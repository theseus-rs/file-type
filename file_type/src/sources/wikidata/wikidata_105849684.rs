use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849684: FileFormat = FileFormat {
    id: 105_849_684,
    puid: "wikidata/105849684",
    name: "Csound unified file format",
    extensions: &["csd"],
    media_types: &["audio/csound"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x73, 0x6F, 0x75, 0x6E, 0x64, 0x53, 0x79, 0x6E, 0x74, 0x68, 0x65,
                    0x73, 0x69, 0x7A, 0x65, 0x72, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
