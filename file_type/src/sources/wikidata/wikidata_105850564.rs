use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850564: FileFormat = FileFormat {
    id: 105_850_564,
    source_type: SourceType::Wikidata,
    name: "Camtasia Studio Project (UTF)",
    extensions: &["camproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
