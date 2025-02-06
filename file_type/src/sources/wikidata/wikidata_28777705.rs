use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777705: FileFormat = FileFormat {
    id: 28_777_705,
    source_type: SourceType::Wikidata,
    name: "MyHeritage Family Tree Builder",
    extensions: &["zed"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
