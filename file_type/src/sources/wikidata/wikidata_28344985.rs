use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344985: FileFormat = FileFormat {
    id: 28_344_985,
    source_type: SourceType::Wikidata,
    name: "Genital Save State",
    extensions: &["gns"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
