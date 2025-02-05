use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850581: FileFormat = FileFormat {
    id: 105_850_581,
    source_type: SourceType::Wikidata,
    name: "HxC Floppy Emulator configuration (generic)",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x58, 0x43, 0x46, 0x45, 0x43, 0x46, 0x47, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
