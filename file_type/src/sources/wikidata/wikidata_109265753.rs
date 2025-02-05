use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109265753: FileFormat = FileFormat {
    id: 109_265_753,
    source_type: SourceType::Wikidata,
    name: "PagePlus Template",
    extensions: &["ppt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
