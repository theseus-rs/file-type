use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344985: FileFormat = FileFormat {
    id: 28_344_985,
    source_type: SourceType::Wikidata,
    name: "Genital Save State",
    extensions: &["gns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
