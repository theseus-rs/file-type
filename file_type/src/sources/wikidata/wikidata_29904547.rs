use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904547: FileFormat = FileFormat {
    id: 29_904_547,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System catalog",
    extensions: &["sas7bcat", "sc2", "sc7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
