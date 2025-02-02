use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27473293: FileFormat = FileFormat {
    id: 27_473_293,
    source_type: SourceType::Wikidata,
    name: "CADRG Overview Image",
    extensions: &["ovr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
