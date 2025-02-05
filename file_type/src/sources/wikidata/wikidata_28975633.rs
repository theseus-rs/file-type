use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975633: FileFormat = FileFormat {
    id: 28_975_633,
    source_type: SourceType::Wikidata,
    name: "NextEngine Scan",
    extensions: &["scn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
