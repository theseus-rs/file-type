use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113495271: FileFormat = FileFormat {
    id: 113_495_271,
    source_type: SourceType::Wikidata,
    name: "Applet Effect Factory Config File",
    extensions: &["data"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
