use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125692441: FileFormat = FileFormat {
    id: 125_692_441,
    source_type: SourceType::Wikidata,
    name: "Microsoft PowerPoint Presentation Template",
    extensions: &["potx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
