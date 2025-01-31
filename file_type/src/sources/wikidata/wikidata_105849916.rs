use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849916: FileFormat = FileFormat {
    id: 105_849_916,
    puid: "wikidata/105849916",
    name: "Compo composition",
    extensions: &["cpo"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x73, 0x69, 0x74, 0x69, 0x6F, 0x6E,
                    0x0A, 0x23, 0x20, 0x53, 0x61, 0x76, 0x65, 0x64, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
