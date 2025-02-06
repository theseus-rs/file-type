use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125150942: FileFormat = FileFormat {
    id: 125_150_942,
    source_type: SourceType::Wikidata,
    name: "OmniGraffle Drawing (zipped)",
    extensions: &["graffle.zip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
