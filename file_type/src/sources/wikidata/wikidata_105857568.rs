use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857568: FileFormat = FileFormat {
    id: 105_857_568,
    source_type: SourceType::Wikidata,
    name: "blueMSX machine settings",
    extensions: &["ini"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
