use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_90406874: FileFormat = FileFormat {
    id: 90_406_874,
    source_type: SourceType::Wikidata,
    name: "QuickTake format",
    extensions: &["qtk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
