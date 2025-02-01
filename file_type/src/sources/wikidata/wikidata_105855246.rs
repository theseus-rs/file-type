use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855246: FileFormat = FileFormat {
    id: 105_855_246,
    puid: "wikidata/105855246",
    name: "Floppy Disk Manager disk image",
    extensions: &["fdm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1A, 0x46, 0x44, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
