use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48814699: FileFormat = FileFormat {
    id: 48_814_699,
    source_type: SourceType::Wikidata,
    name: "DesignCAD for Windows Drawing",
    extensions: &["dw2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
