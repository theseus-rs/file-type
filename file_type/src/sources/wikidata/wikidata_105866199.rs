use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866199: FileFormat = FileFormat {
    id: 105_866_199,
    puid: "wikidata/105866199",
    name: "Floppy Disk Emulator configuration",
    extensions: &["par"],
    media_types: &["text/play"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x52, 0x56, 0x5F, 0x53, 0x45, 0x4C, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
