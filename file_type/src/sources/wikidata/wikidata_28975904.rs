use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975904: FileFormat = FileFormat {
    id: 28_975_904,
    source_type: SourceType::Wikidata,
    name: "Specified points for body pressure file",
    extensions: &["bpi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
