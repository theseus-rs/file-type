use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27823992: FileFormat = FileFormat {
    id: 27_823_992,
    source_type: SourceType::Wikidata,
    name: "Maptech BSB documentation file, version 3.0",
    extensions: &["bsb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
