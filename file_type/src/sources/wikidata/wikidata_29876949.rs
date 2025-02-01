use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29876949: FileFormat = FileFormat {
    id: 29_876_949,
    puid: "wikidata/29876949",
    name: "Clustal W",
    extensions: &["aln"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4C, 0x55, 0x53, 0x54, 0x41, 0x4C, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
