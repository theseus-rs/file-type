use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65990344: FileFormat = FileFormat {
    id: 65_990_344,
    source_type: SourceType::Wikidata,
    name: "Adobe Premiere Project",
    extensions: &["ppj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
