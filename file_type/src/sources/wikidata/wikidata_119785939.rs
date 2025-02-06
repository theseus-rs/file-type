use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119785939: FileFormat = FileFormat {
    id: 119_785_939,
    source_type: SourceType::Wikidata,
    name: "MasterCook Search File",
    extensions: &["src"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
