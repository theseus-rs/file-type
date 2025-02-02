use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34312565: FileFormat = FileFormat {
    id: 34_312_565,
    source_type: SourceType::Wikidata,
    name: "Macromedia Director, compressed PC variant",
    extensions: &["dcr"],
    media_types: &["application/x-director"],
    internal_signatures: &[],
    related_formats: &[],
};
