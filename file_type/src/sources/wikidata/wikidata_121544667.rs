use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121544667: FileFormat = FileFormat {
    id: 121_544_667,
    source_type: SourceType::Wikidata,
    name: "At Home 2009 Tax Return File",
    extensions: &["t09"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
