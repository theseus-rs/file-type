use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551372: FileFormat = FileFormat {
    id: 28_551_372,
    source_type: SourceType::Wikidata,
    name: "Adobe Monitor Setup File",
    extensions: &["ams"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
