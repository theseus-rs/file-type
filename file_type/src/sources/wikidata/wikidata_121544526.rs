use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121544526: FileFormat = FileFormat {
    id: 121_544_526,
    source_type: SourceType::Wikidata,
    name: "At Home 2010 Tax Return File",
    extensions: &["t10"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
