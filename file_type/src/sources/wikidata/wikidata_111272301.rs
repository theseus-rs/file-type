use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272301: FileFormat = FileFormat {
    id: 111_272_301,
    source_type: SourceType::Wikidata,
    name: "Ensoniq ASR instrument file",
    extensions: &["efa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
