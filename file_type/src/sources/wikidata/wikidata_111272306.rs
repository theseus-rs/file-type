use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272306: FileFormat = FileFormat {
    id: 111_272_306,
    source_type: SourceType::Wikidata,
    name: "Ensoniq EPS instrument file",
    extensions: &["efe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
