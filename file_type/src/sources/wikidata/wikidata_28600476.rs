use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600476: FileFormat = FileFormat {
    id: 28_600_476,
    source_type: SourceType::Wikidata,
    name: "DOS device driver",
    extensions: &["dos", "sys"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
