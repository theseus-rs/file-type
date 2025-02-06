use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65990735: FileFormat = FileFormat {
    id: 65_990_735,
    source_type: SourceType::Wikidata,
    name: "Adobe Premiere Library",
    extensions: &["plb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
