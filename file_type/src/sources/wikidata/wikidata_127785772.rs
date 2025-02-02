use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127785772: FileFormat = FileFormat {
    id: 127_785_772,
    source_type: SourceType::Wikidata,
    name: "MEL script",
    extensions: &["mel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
