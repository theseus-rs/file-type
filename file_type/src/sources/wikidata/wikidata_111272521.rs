use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272521: FileFormat = FileFormat {
    id: 111_272_521,
    source_type: SourceType::Wikidata,
    name: "Ensoniq VFX-SD instrument file",
    extensions: &["efv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
