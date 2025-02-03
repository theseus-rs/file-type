use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117313902: FileFormat = FileFormat {
    id: 117_313_902,
    source_type: SourceType::Wikidata,
    name: "Clear Text CGM",
    extensions: &["ctm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
