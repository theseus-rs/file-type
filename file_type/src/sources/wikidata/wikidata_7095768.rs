use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7095768: FileFormat = FileFormat {
    id: 7_095_768,
    source_type: SourceType::Wikidata,
    name: "OpenDRIVE",
    extensions: &["xodr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
