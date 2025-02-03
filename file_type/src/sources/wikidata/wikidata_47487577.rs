use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47487577: FileFormat = FileFormat {
    id: 47_487_577,
    source_type: SourceType::Wikidata,
    name: "Alias Scene Description Language",
    extensions: &["sdl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
