use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48810278: FileFormat = FileFormat {
    id: 48_810_278,
    source_type: SourceType::Wikidata,
    name: "DesignCAD Drawing",
    extensions: &["dc", "dc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
