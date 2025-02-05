use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119785986: FileFormat = FileFormat {
    id: 119_785_986,
    source_type: SourceType::Wikidata,
    name: "MasterCook Calendar File",
    extensions: &["mcl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
