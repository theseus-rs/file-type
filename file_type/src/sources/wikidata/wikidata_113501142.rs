use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113501142: FileFormat = FileFormat {
    id: 113_501_142,
    source_type: SourceType::Wikidata,
    name: "Cintel Raw Image",
    extensions: &["cri", "dvcc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
