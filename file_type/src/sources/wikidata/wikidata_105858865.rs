use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858865: FileFormat = FileFormat {
    id: 105_858_865,
    puid: "wikidata/105858865",
    name: "Biovision Hierarchy character animation format",
    extensions: &["bvh"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x49, 0x45, 0x52, 0x41, 0x52, 0x43, 0x48, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
