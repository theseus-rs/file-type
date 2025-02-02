use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111262619: FileFormat = FileFormat {
    id: 111_262_619,
    source_type: SourceType::Wikidata,
    name: "Raw Yamaha DX7 32-voice data",
    extensions: &["32", "dx7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
