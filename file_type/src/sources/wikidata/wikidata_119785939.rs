use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119785939: FileFormat = FileFormat {
    id: 119_785_939,
    source_type: SourceType::Wikidata,
    name: "MasterCook Search File",
    extensions: &["src"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
