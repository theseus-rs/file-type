use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112669245: FileFormat = FileFormat {
    id: 112_669_245,
    source_type: SourceType::Wikidata,
    name: "Lightscape Layers",
    extensions: &["lay"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
