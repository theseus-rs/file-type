use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_99973071: FileFormat = FileFormat {
    id: 99_973_071,
    source_type: SourceType::Wikidata,
    name: "OmniPage Document 18",
    extensions: &["opd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
