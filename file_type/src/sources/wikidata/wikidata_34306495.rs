use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34306495: FileFormat = FileFormat {
    id: 34_306_495,
    source_type: SourceType::Wikidata,
    name: "Scifer archive",
    extensions: &["sen"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
