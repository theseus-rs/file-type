use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27823998: FileFormat = FileFormat {
    id: 27_823_998,
    source_type: SourceType::Wikidata,
    name: "Maptech Update Patch File, version 3.0",
    extensions: &["ptc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
