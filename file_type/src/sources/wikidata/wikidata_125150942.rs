use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125150942: FileFormat = FileFormat {
    id: 125_150_942,
    source_type: SourceType::Wikidata,
    name: "OmniGraffle Drawing (zipped)",
    extensions: &["graffle.zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
