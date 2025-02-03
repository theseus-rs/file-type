use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131163238: FileFormat = FileFormat {
    id: 131_163_238,
    source_type: SourceType::Wikidata,
    name: "Stan model file",
    extensions: &["stan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
