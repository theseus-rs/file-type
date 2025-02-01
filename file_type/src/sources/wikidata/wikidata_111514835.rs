use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111514835: FileFormat = FileFormat {
    id: 111_514_835,
    puid: "wikidata/111514835",
    name: "3D Movie Maker",
    extensions: &["3mm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x48, 0x4E, 0x32, 0x20, 0x43, 0x4F, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
