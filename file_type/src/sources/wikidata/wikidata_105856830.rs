use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856830: FileFormat = FileFormat {
    id: 105_856_830,
    puid: "wikidata/105856830",
    name: "GFA-BASIC Atari v2 tokenized source",
    extensions: &["gfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x02, 0x47, 0x66, 0x41, 0x42, 0x41, 0x53, 0x49, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
