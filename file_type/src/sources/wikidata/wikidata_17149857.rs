use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17149857: FileFormat = FileFormat {
    id: 17_149_857,
    source_type: SourceType::Wikidata,
    name: "zone file",
    extensions: &["zone"],
    media_types: &["text/dns"],
    internal_signatures: &[],
    related_formats: &[],
};
