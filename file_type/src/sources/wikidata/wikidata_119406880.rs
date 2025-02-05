use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119406880: FileFormat = FileFormat {
    id: 119_406_880,
    source_type: SourceType::Wikidata,
    name: "ACT! Data File",
    extensions: &["adf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
