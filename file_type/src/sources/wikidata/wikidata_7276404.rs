use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7276404: FileFormat = FileFormat {
    id: 7_276_404,
    source_type: SourceType::Wikidata,
    name: "REX2",
    extensions: &["rex", "rx2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
