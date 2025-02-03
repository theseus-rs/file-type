use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857404: FileFormat = FileFormat {
    id: 105_857_404,
    source_type: SourceType::Wikidata,
    name: "DroidPad custom layout",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
