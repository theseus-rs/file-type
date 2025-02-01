use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866371: FileFormat = FileFormat {
    id: 105_866_371,
    puid: "wikidata/105866371",
    name: "Pxlab experiment Design",
    extensions: &["pxd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6D, 0x65, 0x6E, 0x74, 0x28, 0x29,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
