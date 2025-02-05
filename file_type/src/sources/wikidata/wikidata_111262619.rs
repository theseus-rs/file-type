use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111262619: FileFormat = FileFormat {
    id: 111_262_619,
    source_type: SourceType::Wikidata,
    name: "Raw Yamaha DX7 32-voice data",
    extensions: &["32", "dx7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
