use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975870: FileFormat = FileFormat {
    id: 28_975_870,
    source_type: SourceType::Wikidata,
    name: "OOGL INST file",
    extensions: &["inst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
