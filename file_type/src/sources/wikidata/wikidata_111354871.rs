use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111354871: FileFormat = FileFormat {
    id: 111_354_871,
    source_type: SourceType::Wikidata,
    name: "Steinberg LM-4 banks",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
